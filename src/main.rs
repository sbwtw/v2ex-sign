
extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate clap;

use std::io::Read;

use clap::App;
use clap::Arg;
use regex::Regex;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper::header::Headers;
use hyper::status::StatusCode;
use hyper_native_tls::NativeTlsClient;

lazy_static!{
    static ref CLIENT: Client =
        Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));
}

macro_rules! read_file {
    ($file:expr) => {{
        use std::fs::File;
        let mut file = File::open($file).unwrap();
        let mut buf = String::new();

        file.read_to_string(&mut buf).unwrap();

        buf
    }};
}

#[allow(unused_macros)]
macro_rules! save_file {
    ($file:expr, $content:expr) => {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create($file).unwrap();

        file.write_all($content).unwrap();
    };
}

fn main() {

    let matches = App::new("v2ex-sign")
        .author("sbw <sbw@sbw.so>")
        .version("0.0.1")
        .about("v2ex sign")
        .arg(Arg::with_name("cookie")
                 .short("c")
                 .takes_value(true)
                 .default_value("cookie")
                 .help("cookie file"))
        .get_matches();

    let cookie_file = matches.value_of("cookie").unwrap();
    println!("use cookie file: {}", cookie_file);

    let url = "https://www.v2ex.com/mission/daily";
    let cookie = read_file!(cookie_file).trim().to_owned();
    println!("use cookie: {}", cookie);

    let mut headers = Headers::new();
    headers.set_raw("Cookie", vec![cookie.into_bytes()]);
    headers.set_raw("Host", vec![b"www.v2ex.com".to_vec()]);

    let mut response = CLIENT.get(url).headers(headers.clone()).send().unwrap();
    let mut buf = String::new();
    response.read_to_string(&mut buf).unwrap();

    let regex = Regex::new(r#"/mission/daily/redeem\?once=\d+"#).unwrap();
    let caps = match regex.captures(&buf) {
        Some(caps) => caps,
        _ => {
            println!("sign url not found!");
            return;
        }
    };

    let url = caps.get(0).unwrap().as_str();
    let url = format!("https://www.v2ex.com{}", url);

    let response = CLIENT.get(&url).headers(headers).send().unwrap();
    if response.status == StatusCode::Ok {
        println!("success");
    } else {
        println!("fail, {:#?}", response);
    }

    // save_file!("/tmp/1.txt", buf.as_bytes());
}

