use std::io::Write;
use std::str::FromStr;
use bytesize::ByteSize;
use clap::{Parser, Subcommand};
use reolink_api::api::record::download::DownloadRequest;
use reolink_api::api::record::search::{Search, SearchRequest};
use reolink_api::api::record::snapshot::SnapshotRequest;
use reolink_api::{chrono, ReolinkBlockingClient};
use reolink_api::api::Channel;
use reolink_api::chrono::{NaiveDateTime, NaiveTime, TimeDelta};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Recordings { start, end, channel, quiet } => {

            let client = get_client()?;
            let mut files_found = false;

            for (start, end) in day_intervals(start, end) {
                let result = client.exec(&SearchRequest {
                    search: Search {
                        channel: channel,
                        only_status: false,
                        stream_type: "main".to_string(),
                        start_time: start.into(),
                        end_time: end.into(),
                    },
                })?;

                let files = result.search_result.file.unwrap_or(Vec::new());
                files_found |= !files.is_empty();
                for file in files {
                    if quiet {
                        println!("{}", file.name);
                    } else {
                        let start = NaiveDateTime::from(file.start_time);
                        let duration = (NaiveDateTime::from(file.end_time) - start).num_seconds();
                        println!("{} - {} s - {}: {}", start, duration, ByteSize(file.size as u64), file.name);
                    }
                }
                if !files_found {
                    eprintln!("No files found");
                }
            }
        }

        Commands::Download { file } => {
            let client = get_client()?;
            let bytes = client.download(&DownloadRequest {
                source: file,
                output: None,
            })?;

            std::io::stdout().write_all(&bytes)?;
        }

        Commands::Snapshot { channel } => {
            let client = get_client()?;

            let bytes = client.download(&SnapshotRequest {
                channel: channel,
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
        /// Start time as YYYY-MM-ddThh:mm:ss or number of days ago
        #[arg(short, long, value_parser(parse_start_date))]
        start: NaiveDateTime,
        /// End time as YYYY-MM-ddThh:mm:ss or number of days ago [default: end of <start> day]
        #[arg(short, long, value_parser(parse_end_date))]
        end: Option<NaiveDateTime>,
        #[arg(short, long, default_value = "0")]
        channel: Channel,
        /// Quiet mode, list only filenames
        #[arg(short, long)]
        quiet: bool,
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

const START_OF_DAY: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
const ONE_SECOND: TimeDelta = TimeDelta::seconds(1);
const ONE_DAY: TimeDelta = TimeDelta::days(1);

fn end_of_day(datetime: NaiveDateTime) -> NaiveDateTime {
    start_of_next_day(datetime) - ONE_SECOND
}

fn start_of_day(datetime: NaiveDateTime) -> NaiveDateTime {
    NaiveDateTime::new(datetime.date(), START_OF_DAY)
}

fn start_of_next_day(datetime: NaiveDateTime) -> NaiveDateTime {
    start_of_day(datetime) + ONE_DAY
}

fn day_intervals(mut start: NaiveDateTime, end: Option<NaiveDateTime>) -> Vec<(NaiveDateTime, NaiveDateTime)> {
    let end = end.unwrap_or_else(|| end_of_day(start));

    let mut current_end;
    let mut result = Vec::new();

    while { current_end = end_of_day(start); current_end } < end {
        result.push((start, current_end));
        start = start_of_next_day(start);
    }

    result.push((start, end));
    result
}

fn parse_start_date(date: &str) -> chrono::format::ParseResult<NaiveDateTime> {
    if let Ok(days) = str::parse::<i64>(date) {
        let today = chrono::offset::Local::now().naive_local().date();
        Ok(NaiveDateTime::new(today - TimeDelta::days(days), START_OF_DAY))
    } else {
        NaiveDateTime::from_str(date)
    }
}

fn parse_end_date(date: &str) -> chrono::format::ParseResult<NaiveDateTime> {
    if let Ok(days) = str::parse::<i64>(date) {
        let today = chrono::offset::Local::now().naive_local().date();
        // Start of next day minus 1 second
        Ok(NaiveDateTime::new(today - TimeDelta::days(days - 1), START_OF_DAY) - ONE_SECOND)
    } else {
        NaiveDateTime::from_str(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use reolink_api::chrono::Datelike;

    #[test]
    fn test_end_of_day() -> anyhow::Result<()> {
        let date = NaiveDateTime::from_str("2025-02-09T10:21:32")?;
        let end = end_of_day(date);
        assert_eq!(2025, end.year());
        assert_eq!(2, end.month());
        assert_eq!(9, end.day());
        assert_eq!(23, end.hour());
        assert_eq!(59, end.minute());
        assert_eq!(59, end.second());
        Ok(())
    }

    #[test]
    fn test_start_of_next_day() -> anyhow::Result<()> {
        let date = NaiveDateTime::from_str("2025-02-09T10:21:32")?;
        let end = start_of_next_day(date);
        assert_eq!(2025, end.year());
        assert_eq!(2, end.month());
        assert_eq!(10, end.day());
        assert_eq!(0, end.hour());
        assert_eq!(0, end.minute());
        assert_eq!(0, end.second());
        Ok(())
    }
}
