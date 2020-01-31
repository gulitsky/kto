use clap::{App, AppSettings, Arg};
use reqwest::{Client, Proxy};

mod num_buster;

const USER_AGENT: &'static str = "okhttp/3.9.1";
const API_ENDPOINT: &'static str = "https://api.numbuster.com/api/";

fn main() {
    let matches = App::new("NumBuster!")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("authenticate")
                .short("a")
                .long("authenticate")
                .help("Interactively get access token through SMS authentication"),
        )
        .arg(
            Arg::with_name("access_token")
                .short("t")
                .long("access-token")
                .takes_value(true)
                .value_name("ACCESS_TOKEN")
                .required_unless("authenticate"),
        )
        .arg(
            Arg::with_name("proxy_address")
                .short("P")
                .long("proxy-address")
                .takes_value(true)
                .value_name("PROXY_ADDRESS")
                .validator(validate_proxy_address),
        )
        .arg(
            Arg::from_usage("<phone_number> <PHONE_NUMBER> 'Phone number in E.164 format, e.g., +15557654321'")
                .last(true)
                .validator(validate_phone_number),
        )
        .setting(AppSettings::TrailingVarArg)
        .get_matches();

    let mut client_builder = Client::builder().user_agent(USER_AGENT);
    if let Some(proxy_address) = matches.value_of("proxy_address") {
        let proxy = Proxy::all(proxy_address).unwrap();
        client_builder = client_builder.proxy(proxy);
    }
    let _client: Client = client_builder.build().unwrap();
}

fn validate_proxy_address(proxy_address: String) -> Result<(), String> {
    use url::Url;

    Url::parse(proxy_address.as_str())
        .map_err(|e| e.to_string())
        .and_then(|url| {
            if vec!["http", "https", "socks5"].contains(&url.scheme()) {
                Ok(())
            } else {
                Err(String::from(
                    "only HTTP, HTTPS, or SOCKS5 proxies are supported",
                ))
            }
        })
}

fn validate_phone_number(phone_number: String) -> Result<(), String> {
    use regex::Regex;

    Regex::new(r"^\+[1-9]\d{1,14}$")
        .map_err(|e| e.to_string())
        .and_then(|re| {
            if re.is_match(phone_number.as_str()) {
                Ok(())
            } else {
                Err(String::from("phone number should be in E.164 format, e.g., +15557654321"))
            }
        })
}
