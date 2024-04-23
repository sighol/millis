use chrono::prelude::*;
use chrono::DateTime;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
struct App {
    millis: Option<f64>,

    #[arg(short = 'l', long = "local", help = "Use local timezone")]
    use_local_timezone: bool,
}

fn main() {
    let args = App::parse();

    if let Some(millis) = args.millis {
        let utc = DateTime::from_timestamp_millis(millis as i64).unwrap();

        if args.use_local_timezone {
            let local = utc.with_timezone(&Local);
            println!("{local:?}")
        } else {
            println!("{utc:?}");
        }
    } else {
        let now = Utc::now();
        println!("{}", now.timestamp_millis())
    }
}
