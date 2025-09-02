use clap::Parser;
use zbus::connection::Connection;

#[derive(Parser, Debug)]
#[command(about = "Add watcher for status watcher")]
struct Args {
    #[arg(
        index = 1,
        help = "Specify the target to watch,\nif connection to this target fails, connection is retried after a minute\n"
    )]
    target_url: Option<String>,

    #[arg(
        long = "url",
        short = 'l',
        help = "Does the exact same thing as setting target url\nJust a bit more formal, lol\n"
    )]
    url: Option<String>,

    #[arg(
        long = "alias",
        help = "Sets an alias for the target url to watch,\nAll the following events will use this name instead of \nmentioning the whole url\n"
    )]
    name: Option<String>,
}

impl Args {
    fn check_and_return(&self) -> &str {
        if self.url.is_some() && self.target_url.is_some() {
            eprintln!(
                "[\x1b[33m{}\x1b[0m] or [\x1b[33m{}\x1b[0m]\n\x1b[33mMore than one target url mentioned, \x1b[1mONLY ONE\x1b[29m was expected\x1b[0m\nEither use \x1b[1m--url\x1b[0m or don't, or maybe use some \x1b[1m--help\x1b[0m",
                self.url.as_ref().unwrap(),
                self.target_url.as_ref().unwrap()
            );
            std::process::exit(2);
        } else if self.url.is_none() && self.target_url.is_none() {
            eprintln!("\x1b[33m[TARGET URL] was expected... found none\x1b[0m\nUse \x1b[1m--help\x1b[0m for usage and help");
            std::process::exit(0);
        } else {
            if self.url.is_none() { self.target_url.as_ref().unwrap() } else { self.url.as_ref().unwrap() }
        }
    }
}

fn main() {
    let args = Args::parse();
    let url = args.check_and_return().to_owned();
    smol::block_on(async move {
        let connection = Connection::session().await.expect("Failed creating a session");

        let task = connection
            .call_method(
                Some("org.status_watcher.Daemon"),
                "/org/status_watcher/Daemon",
                Some("org.status_watcher.Daemon"),
                "SetWatcher",
                &(
                    url.clone(),
                    args.name.unwrap_or(String::new()),
                    false,
                ),
            )
            .await;

        if let Ok(_) = task {
            println!("Added {} to watchlist!", url);
        } else {
            eprintln!("\nCan't find watcher daemon... is \x1b[1mstatus_watcher\x1b[0m running?");
            std::process::exit(1);
        }
    });
}
