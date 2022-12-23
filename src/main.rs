use chrono::Local;
use clap::Parser;
use ezz::client::Zoom;
use ezz::{date, time, Meeting};

fn parse_date(value: &str) -> Result<String, std::io::Error> {
    date::parse(Local::now().naive_local().date(), value)
}

#[derive(Parser, Default)]
#[command(about = "ezz is a simple CLI tool to schedule Zoom meetings.")]
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

fn main() {
    let args = Args::parse();
    let meeting = Meeting::new(
        args.name,
        args.password,
        args.timezone,
        args.on,
        args.at,
        args.duration,
    );
    let client = match Zoom::new() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    match client.save(&meeting) {
        Ok(url) => println!("{}", url),
        Err(e) => {
            eprintln!("Error: {}", e.message);
            std::process::exit(1);
        }
    }
}
