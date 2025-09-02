use clap::Parser;
use reqwest::Client;
use std::future::pending;
use tokio::sync::mpsc::{Sender, channel};
use tokio::time::{Duration, sleep};
use zbus::{connection, interface};

#[derive(Parser, Debug)]
#[command(about = "Start status_watcher daemon")]
struct Args {}

struct Watcher {
    handle: tokio::runtime::Handle,
    sender: tokio::sync::mpsc::Sender<String>,
}

#[allow(non_snake_case)]
#[interface(name = "org.status_watcher.Daemon")]
impl Watcher {
    async fn SetWatcher(&mut self, url: String, alias: String, keep_watching: bool) {
        let name = if alias.is_empty() { url.clone() } else { alias };
        let sender = self.sender.clone();
        self.handle.spawn({
            async move {
                loop {
                    match reqwest::get(&url).await {
                        Ok(_) => {
                            sender.send(name.clone()).await.unwrap();
                            break;
                        }
                        Err(_) => {
                            #[cfg(debug_assertions)]
                            {
                                eprintln!(
                                    "\x1b[31mRequest failed!! {} failed to connect\x1b[0m",
                                    &name
                                );
                            }
                            sleep(Duration::from_secs(60)).await;
                        }
                    }
                }
            }
        });
    }

    fn NoHello(&self) {
        println!("Nigga didn't even say hello")
    }
}

impl Watcher {
    #[inline]
    fn new(handle: tokio::runtime::Handle, sender: Sender<String>) -> Self {
        Watcher { handle, sender }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let handle = tokio::runtime::Handle::current();

    let (tx, mut rx) = channel::<String>(2048);
    let watcher = Watcher::new(handle, tx);

    let connection = connection::Builder::session()
        .unwrap()
        .name("org.status_watcher.Daemon")
        .unwrap()
        .serve_at("/org/status_watcher/Daemon", watcher)
        .unwrap()
        .build()
        .await
        .unwrap();

    tokio::spawn(async move {
        while let Some(x) = rx.recv().await {
            println!(
                "{{\"text\": \"{x} is live\", \"alt\" : \"active\", \"tooltip\": \"$tooltip\", \"class\": \"active\", \"percentage\": 0 }}"
            );
            sleep(Duration::from_secs(20)).await;
            println!(
                "{{\"text\": \"\", \"alt\" : \"default\", \"tooltip\": \"$tooltip\", \"class\": \"default\", \"percentage\": 0 }}"
            )
        }
    });

    pending::<()>().await;
}

async fn handle_response<T>() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {})
}

async fn check_status(url: &str) -> Result<u16, reqwest::Error> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();
    let response = client.head(url).send().await?;

    Ok(response.status().as_u16())
}

async fn handle_ok_response<T: AsRef<str> + std::fmt::Display>(str: T) {
    println!(
        "{{\"text\": \"{} is live!\", \"tooltip\": \"$tooltip\", \"class\": \"active\", \"percentage\": $percentage, alt : \"active\" }}",
        str
    );
    tokio::time::sleep(Duration::from_secs(20)).await;
    println!(
        "{{\"text\": \"\", \"tooltip\": \"$tooltip\", \"class\": \"default\", \"percentage\": $percentage }}"
    )
}
