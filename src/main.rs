mod utils;

use anyhow::{Context,Result};
use log::info;
use structopt::StructOpt;
use tokio_cron_scheduler::{Job,JobScheduler};
use utils::logger;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "elasticsearch-alerter",
    author = "Cenk Kilic <cenk@kilic.dev>",
    version = "v1.0.0",
    about = "elastic",
    long_about = "something"
)]
struct Command {
    #[structopt(long, env, help = "Log level for the application.", default_value = "info", required = false)]
    log_level: String,

    #[structopt(long, short, env, help = "Interval to check for new alerts.", default_value = "1m", required = false)]
    interval: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Command::clap();
    let opt = Command::from_args();

    let app_name = app.get_name();

    println!("{}", app_name);
    println!("{}", "-".repeat(app_name.len()));

    logger::setup_logger(opt.log_level.as_str())?;

    let mut scheduler = JobScheduler::new();

    scheduler.add(Job::new_repeated(duration_str::parse(&opt.interval).with_context(|| format!("Can not parse the given time interval.")).unwrap(), |_uuid, _l| {
      info!("I'm repeated every 8 seconds");
    }).unwrap()).unwrap();

    scheduler.start().await.with_context(|| format!("Can not start background jobs."))?;

    Ok(())
}
