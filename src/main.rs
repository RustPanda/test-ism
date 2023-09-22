use std::time::Duration;

use clap::Parser;
use reqwest::Url;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    /// Целочисленное значение интервала в секундах
    #[arg(value_parser = interval_parser)]
    interval: Duration,
    /// HTTP URL который будет проверяться
    #[arg(value_parser = url_parser)]
    url: Url,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Получаю аргументы коман
    let opts = Opts::parse();

    println!("{opts:#?}");
}

// Парсер интервала
fn interval_parser(interval: &str) -> Result<Duration, String> {
    interval
        .parse::<f32>()
        .map_err(|_| ())
        .and_then(|i| Duration::try_from_secs_f32(i).map_err(|_| ()))
        .map_err(|_| "Interval parsing error".to_string())
}

// Парсер URL
fn url_parser(url: &str) -> Result<Url, String> {
    Url::parse(url).map_err(|_| "URL parsing error".to_string())
}
