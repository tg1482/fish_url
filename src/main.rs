use std::env;
use serde_json::Value;
use reqwest::blocking::Client;
use regex::Regex;

const CONFIG: &str = r#"
{
    "services": [
        {
            "name": "Google",
            "url": "https://webcache.googleusercontent.com/search?q=cache:",
            "error": "This page appears to have been removed"
        },
        {
            "name": "Freedium",
            "url": "https://freedium.cfd/",
            "error": "Please check the URL for any typing errors."
        },
        {
            "name": "Archive",
            "url": "https://web.archive.org/web/",
            "error": "Wayback Machine doesn't have that page archived"
        },
        {
            "name": "Ghostarchive",
            "url": "https://ghostarchive.org/search?term=",
            "error": "No archives for that site."
        }
    ]
}
"#;

fn check_url(client: &Client, url: &str, service: &str, error_msg: &str) -> bool {
    println!("Checking {}...", service);
    match client.get(url).send() {
        Ok(response) => {
            println!("Response code: {}", response.status());
            if response.status().is_success() {
                match response.text() {
                    Ok(body) => {
                        if body.contains(error_msg) {
                            println!("{}: Error found", service);
                            false
                        } else {
                            true
                        }
                    }
                    Err(_) => {
                        println!("Failed to read response body");
                        false
                    }
                }
            } else {
                println!("{} returned non-200 status code", service);
                false
            }
        }
        Err(e) => {
            println!("Request failed: {}", e);
            false
        }
    }
}

fn add_www(url: &str) -> String {
    let re = Regex::new(r"^https?://www\.").unwrap();
    if re.is_match(url) {
        url.to_string()
    } else if url.starts_with("http://") || url.starts_with("https://") {
        url.replace("://", "://www.")
    } else {
        format!("https://www.{}", url)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <link>", args[0]);
        std::process::exit(1);
    }

    let original_url = add_www(&args[1]);
    println!("Using URL: {}", original_url);

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36")
        .build()
        .unwrap();

    let config: Value = serde_json::from_str(CONFIG).unwrap();
    let services = config["services"].as_array().unwrap();

    for service in services {
        let name = service["name"].as_str().unwrap();
        let url = service["url"].as_str().unwrap();
        let error = service["error"].as_str().unwrap();

        let cache_url = format!("{}{}", url, original_url);
        if check_url(&client, &cache_url, name, error) {
            println!("{} version available: {}", name, cache_url);
            std::process::exit(0);
        } else {
            println!("{} version not available or blocked.", name);
            println!();
        }
    }

    println!("No cached or unblocked version found.");
    std::process::exit(1);
}