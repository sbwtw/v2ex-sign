use std::error::Error;

use clap::App;
use clap::Arg;
use cookies_rs::load_cookies;
use regex::Regex;
use reqwest::header::HeaderMap;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("v2ex-sign")
        .author("sbw <sbw@sbw.so>")
        .version("0.0.1")
        .about("v2ex sign")
        .arg(
            Arg::with_name("cookie")
                .short("c")
                .takes_value(true)
                .required(true)
                .help("cookie file"),
        )
        .get_matches();

    let cookie_file = matches.value_of("cookie").unwrap();
    println!("use cookie file: {}", cookie_file);

    let jar = load_cookies!(cookie_file).unwrap();
    let mut cookie_string = String::new();
    for cookie in jar.iter() {
        if cookie.domain().map(|x| x.contains("v2ex.com")) == Some(true) {
            let (name, value) = cookie.name_value();
            cookie_string.push_str(&format!("{}={}; ", name, value));
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Cookie", cookie_string.parse().unwrap());
    headers.insert("Host", "www.v2ex.com".parse().unwrap());

    let client = Client::new();
    let url = "https://www.v2ex.com/mission/daily";
    let body = client
        .get(url)
        .headers(headers.clone())
        .send()
        .await?
        .text()
        .await?;

    let login_test = Regex::new(r#">登出</a>"#).unwrap();
    let is_login = login_test.find(&body).is_some();

    if !is_login {
        println!("Not Login, maybe cookie is expired");
        return Ok(());
    }

    let regex = Regex::new(r#"/mission/daily/redeem\?once=\d+"#).unwrap();
    let caps = match regex.captures(&body) {
        Some(caps) => caps,
        _ => {
            println!("mission url not found!");
            return Ok(());
        }
    };

    let url = caps.get(0).unwrap().as_str();
    let url = format!("https://www.v2ex.com{}", url);
    println!("{}", url);

    let response = client.get(&url).headers(headers.clone()).send().await?;
    println!("Response status: {}", response.status().as_u16());

    Ok(())
}
