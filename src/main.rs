
use std::io::Read;

use clap::App;
use clap::Arg;
use regex::Regex;
use reqwest::Client;
use reqwest::header::HeaderMap;

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
        .arg(
            Arg::with_name("cookie")
                .short("c")
                .takes_value(true)
                .default_value("cookie")
                .help("cookie file"),
        )
        .get_matches();

    let cookie_file = matches.value_of("cookie").unwrap();
    println!("use cookie file: {}", cookie_file);

    let cookie = read_file!(cookie_file).trim().to_owned();
    let mut headers = HeaderMap::new();
    headers.insert("Cookie", cookie.parse().unwrap());
    headers.insert("Host", "www.v2ex.com".parse().unwrap());

    let client = Client::new();
    let url = "https://www.v2ex.com/mission/daily";
    let mut response = client.get(url).headers(headers.clone()).send().unwrap();
    let mut buf = String::new();
    response.read_to_string(&mut buf).unwrap();

    let login_test = Regex::new(r#">登出</a>"#).unwrap();
    let is_login = login_test.find(&buf).is_some();

    if !is_login {
        println!("Not Login, exit.");
        return;
    }


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
    println!("{}", url);

    client.get(&url).headers(headers).send().unwrap();
}
