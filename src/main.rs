use chrono::Local;
use clap::Parser;
use ezz::client::{ClientError, Zoom};
use ezz::{date, time, Meeting};

fn parse_date(value: &str) -> Result<String, std::io::Error> {
    date::parse(Local::now().naive_local().date(), value)
}

/// ezz is a simple CLI tool to schedule Zoom meetings.
#[derive(Parser, Default)]
#[command()]
struct Args {
    /// Name of the meeting
    #[arg(short, long)]
    name: String,

    #[arg(
        short,
        long,
        name = "DATE",
        value_parser = parse_date,
        help = format!(
            "Date of the meeting in {} format or one of: {}",
            date::HUMAN_FORMAT,
            date::ALIASES.join(", "),
        ),
    )]
    on: String,

    #[arg(
        short,
        long,
        name = "TIME",
        value_parser = time::parse,
        help= format!("Time of the meeting in {} format", time::HUMAN_FORMAT),
    )]
    at: String,

    /// Duration of the meeting in minutes
    #[arg(short, long, default_value = "60")]
    duration: u16,

    /// Meeting password, max. 10 characters [default: auto-generated random password]
    #[arg(short, long)]
    password: Option<String>,

    /// Timezone (as in https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists/#timezones) for the meeting, e.g. America/Recife [default: your account's timezone]
    #[arg(short, long)]
    timezone: Option<String>,
}

fn main() -> Result<(), ClientError> {
    let args = Args::parse();
    let meeting = Meeting::new(
        args.name,
        args.password,
        args.timezone,
        args.on,
        args.at,
        args.duration,
    );
    let meeting_url = Zoom::new().and_then(|client| client.save(&meeting))?;
    println!("{}", meeting_url);
    Ok(())
}
