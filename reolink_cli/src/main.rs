use std::io::Write;
use clap::{Parser, Subcommand};
use reolink_api::api::record::download::DownloadRequest;
use reolink_api::api::record::search::{Search, SearchRequest};
use reolink_api::api::record::snapshot::SnapshotRequest;
use reolink_api::ReolinkBlockingClient;
use reolink_api::chrono;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Recordings { start, end, channel } => {
            if start.date() != end.date() {
                eprintln!("Start and end must be the same day to have results (to be fixed)");
                std::process::exit(1);
            }

            let client = get_client()?;
            let result = client.exec(&SearchRequest {
                search: Search {
                    channel: channel as usize,
                    only_status: 0,
                    stream_type: "main".to_string(),
                    start_time: start.into(),
                    end_time: end.into(),
                },
            })?;

            let files = result.search_result.file.unwrap_or(Vec::new());
            if files.is_empty() {
                eprintln!("No results found");
            } else {
                for file in files {
                    println!("{}", file.name);
                }
            }
        }

        Commands::Download { file } => {
            let mut client = get_client()?;
            client.use_token()?;

            let bytes = client.download(&DownloadRequest {
                source: file,
                output: None,
            })?;

            std::io::stdout().write_all(&bytes)?;

            client.logout()?; // FIXME: need logout on drop
        }

        Commands::Snapshot { channel } => {
            let client = get_client()?;

            let bytes = client.download(&SnapshotRequest {
                channel: channel as usize,
                rs: "xx".to_string(),
            })?;

            std::io::stdout().write_all(&bytes)?;
        }
    }

    Ok(())
}

fn get_client() -> anyhow::Result<ReolinkBlockingClient> {
    dotenv::dotenv().ok();

    let url = std::env::var("REOLINK_URL")?;
    let login = std::env::var("REOLINK_LOGIN")?;
    let password = std::env::var("REOLINK_PASSWORD")?;

    ReolinkBlockingClient::new(&url, login, password)
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists available recordings
    Recordings {
        /// Start time as YY-MM-ddThh:mm:ss
        #[arg(short, long)]
        start: chrono::NaiveDateTime,
        /// End time as YY-MM-ddThh:mm:ss
        #[arg(short, long)]
        end: chrono::NaiveDateTime,
        #[arg(short, long, default_value = "0")]
        channel: u8,
    },

    /// Download a file
    Download {
        /// File name on the device
        file: String,
    },

    /// Take a snapshot
    Snapshot {
        #[arg(short, long, default_value = "0")]
        channel: u8
    }
}
