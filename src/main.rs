mod circular_buffer;
mod pack;
mod splice;

use clap::Parser;
use env_logger::Env;

use crate::pack::PackListener;

/// A CLI tool that polls services (sites) for new releases and, optionally, sends notifications to a Discord webhook.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The discord webhook to send notifications to (optional)
    #[arg(short, long)]
    webhook: Option<String>,

    /// How often (per second) to poll for new packs on Splice
    #[arg(short = 'p', long, default_value_t = 60)]
    splice_poll_rate: u64,

    /// Disable the Splice listener
    #[arg(short = 's', long, default_value_t = false)]
    disable_splice: bool,
}

/// Some essential data for each listener.
struct ListenerData {
    webhook: Option<String>,
    poll_rate: u64,
}
impl From<&Cli> for ListenerData {
    fn from(cli: &Cli) -> Self {
        Self {
            webhook: cli.webhook.clone(),
            poll_rate: cli.splice_poll_rate,
        }
    }
}

/// Waits for a certain amount of seconds.
async fn wait(seconds: u64) {
    tokio::time::sleep(std::time::Duration::from_secs(seconds)).await;
}

/// Starts a listener.
macro_rules! start_listener {
    ($runtime:expr, $cli:expr, $listener:ty) => {
        let data = ListenerData::from($cli);
        $runtime.spawn(async move {
            let mut listener = <$listener>::new();
            loop {
                match listener.poll_and_notify(data.webhook.as_deref()).await {
                    Ok(0) => log::debug!("No new packs found :("),
                    Ok(n) => log::info!("Found {} new packs!", n),
                    Err(e) => log::error!("An error occurred: {}", e),
                };
                wait(data.poll_rate).await;
            }
        });
    };
}

/// Starts all listeners.
async fn start_listeners(cli: Cli) {
    // Start a listener for each service
    let mut join_set = tokio::task::JoinSet::new();
    if !cli.disable_splice {
        log::info!("Starting the Splice listener...");
        start_listener!(join_set, &cli, splice::SpliceListener);
    }

    // Wait for all listeners to finish
    log::info!("All listeners started.");
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(_) => log::info!("Listener has stopped"),
            Err(e) => log::error!("Listener has stopped with an error: {}", e),
        }
    }
}

fn main() {
    // Initialise the application
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);
    let cli = Cli::parse();

    // Start a listener for each service
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_time()
        .enable_io()
        .build()
        .unwrap()
        .block_on(start_listeners(cli));

    // This should never be reached, if at least one listener has been enabled
    log::info!("Exiting the application, all listeners have stopped?");
}
