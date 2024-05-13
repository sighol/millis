use chrono::prelude::*;
use chrono::DateTime;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
struct Args {
    millis: Option<f64>,

    #[arg(short = 'l', long = "local", help = "Use local timezone")]
    use_local_timezone: bool,

    #[arg(long = "weekday", short = 'w', help = "Print week day")]
    print_week_day: bool,

    #[arg(short, long, help = "Use current time")]
    now: bool,
}

fn main() {
    let args = Args::parse();

    if args.now {
        let utc = Utc::now();
        let utc = utc
            .with_nanosecond(utc.nanosecond() / 1_000_000 * 1_000_000)
            .unwrap();
        print_time(utc, &args);
    } else if let Some(millis) = args.millis {
        let utc = DateTime::from_timestamp_millis(millis as i64).unwrap();
        print_time(utc, &args);
    } else {
        let now = Utc::now();
        println!("{}", now.timestamp_millis())
    }
}

fn print_time(utc: DateTime<Utc>, args: &Args) {
    if args.use_local_timezone {
        let local = utc.with_timezone(&Local);
        if args.print_week_day {
            println!("{} {:?}", local.weekday(), local)
        } else {
            println!("{local:?}")
        }
    } else {
        if args.print_week_day {
            println!("{} {:?}", utc.weekday(), utc)
        } else {
            println!("{utc:?}")
        }
    }
}
