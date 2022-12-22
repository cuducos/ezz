use clap::Parser;

mod date;
mod meeting;
mod responses;
mod settings;
mod time;

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
        value_parser = date::parse_date,
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
        value_parser = time::parse_time,
        help= format!("Time of the meeting in {} format", time::HUMAN_FORMAT),
    )]
    at: String,

    /// Duration of the meeting in minutes
    #[arg(short, long, default_value = "60")]
    duration: u16,

    /// Meeting password (max. 10 characters)
    #[arg(short, long)]
    password: Option<String>,

    /// Timezone (as in https://marketplace.zoom.us/docs/api-reference/other-references/abbreviation-lists/#timezones) for the meeting, e.g. America/Recife [default: your account's timezone]
    #[arg(short, long)]
    timezone: Option<String>,
}

fn main() {
    let args = Args::parse();
    let settings = settings::Settings::from_env();
    let token = settings.get_token();
    let meeting = meeting::Meeting::new(
        args.name,
        args.password,
        args.timezone,
        args.on,
        args.at,
        args.duration,
    );
    println!("{}", meeting.save(token));
}
