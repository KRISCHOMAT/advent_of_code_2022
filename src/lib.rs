use reqwest::Client;
use reqwest::header::COOKIE;
use dotenv;

#[tokio::main]
pub async fn crawl_input(year: &str, day: &str) -> Result<String, reqwest::Error> {
    let key = "TOKEN";
    let token = dotenv::var(key).unwrap();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let client = Client::new();
    let res = client.get(url)
        .header(COOKIE, format!("session={token}"))
        .send()
        .await
        .unwrap()
        .text()
        .await;
    res
}