use std::{io::Write, sync::LazyLock};

use clap::Parser;
use env_logger::{
    fmt::style::{AnsiColor, Style},
    Builder, Env,
};
use gan_robot_controller::GanRobotController;
use jiff::{tz::TimeZone, Zoned};

static TZ: LazyLock<TimeZone> = LazyLock::new(|| TimeZone::get("Asia/Tokyo").unwrap());

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Args {
    /// The name of the GAN robot.
    #[arg(short, long, env = "GAN_ROBOT_NAME", default_value = "GAN-a7f13")]
    pub name: String,

    /// The move characteristic UUID of the GAN robot.
    #[arg(
        short,
        long,
        env = "GAN_ROBOT_MOVE_CHARACTERISTIC",
        default_value = "0000fff3-0000-1000-8000-00805f9b34fb"
    )]
    pub move_characteristic: String,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let subtle = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
            let level_style = buf.default_level_style(record.level());

            writeln!(
                buf,
                "{subtle}[{subtle:#}{} {level_style}{:<5}{level_style:#}{subtle}]{subtle:#} {}",
                Zoned::now()
                    .with_time_zone(TZ.clone())
                    .strftime("%Y-%m-%d %H:%M:%S %:z"),
                record.level(),
                record.args()
            )
        })
        .init();

    let Args { name, move_characteristic } = Args::parse();
    let controller = GanRobotController::try_new(&name, &move_characteristic)?
        .try_connect()
        .await?;

    controller.scramble(8).await?;
    controller.disconnect().await?;

    Ok(())
}
