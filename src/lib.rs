use reqwest::Client;
use reqwest::header::COOKIE;
use dotenv;
use scraper::{Html, Selector};

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

#[tokio::main]
pub async fn send_answer(year: &str, day: &str, level: &str, answer: &str)-> String{
    let key = "TOKEN";
    let token = dotenv::var(key).unwrap();
    let params = [("level", level), ("answer", answer)];
    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let client = Client::new();
    let res = client.post(url)
        .header(COOKIE, format!("session={token}"))
        .form(&params)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let fragment = Html::parse_fragment(&res);
    let selector = Selector::parse("p").unwrap();
    let result = fragment.select(&selector).next().unwrap();

    result.inner_html()
}