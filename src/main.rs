use std::time::Duration;

use clap::Parser;
use reqwest::Url;
use tokio::signal;

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
    let Opts { interval, url } = Opts::parse();

    tokio::select! {
        _ = run_checkloop(url, interval) => {},
        _ = close_signal() => {},
    }
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

// Запуск цикла helthcheck
async fn run_checkloop(url: Url, interval: Duration) {
    let client = reqwest::Client::new();
    let mut intervel = tokio::time::interval(interval);

    loop {
        intervel.tick().await;

        let response = client.get(url.clone()).send().await;

        match response {
            Ok(response) => {
                if response.status().as_u16() == 200 {
                    println!("Checking '{url}'. Result: OK(200)")
                } else {
                    eprintln!(
                        "Checking '{url}'. Result: ERR({code})",
                        code = response.status().as_u16()
                    );
                }
            }
            Err(error) => {
                eprintln!("Failed to check '{url}': {error}")
            }
        }
    }
}

// Асинхронная функция ожидания сигнала ctrl+c
// Скопирована из https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs#L31
async fn close_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("");
}
