use clap::Parser;

mod date;
mod meeting;
mod responses;
mod settings;

#[derive(Parser, Default)]
#[command(about = "ezz is a simple CLI tool to schedule meetings on Zoom.")]
struct Args {
    /// Name of the meeting
    #[arg(short, long)]
    name: String,

    /// Meeting password
    #[arg(short, long)]
    password: String,

    /// Timezone (as in the TZ database) for the meeting, e.g. America/Recife (see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)
    #[arg(short, long, default_value = "Etc/UTC")]
    timezone: String,

    /// Duration of the meeting in minutes
    #[arg(short, long, default_value = "60")]
    duration: u16,

    #[arg(
        short = 'w',
        long,
        value_parser = date::parse_date,
        help = format!(
            "Date of the meeting in {} format or one of: {}",
            date::HUMAN_FORMAT,
            date::ALIASES.join(", "),
        ),
    )]
    date: String,

    /// Time of the meeting in HH:MM format
    #[arg(short = 'a', long)]
    time: String,
}

fn main() {
    let args = Args::parse();
    let settings = settings::Settings::from_env();
    let token = settings.get_token();
    let meeting = meeting::Meeting::new(
        args.name,
        args.password,
        args.timezone,
        args.date,
        args.time,
        args.duration,
    );
    println!("{}", meeting.save(token));
}
