use std::{io::Write, sync::LazyLock};

use env_logger::{
    fmt::style::{AnsiColor, Style},
    Builder, Env,
};
use gan_robot_controller::GanRobotController;
use jiff::{tz::TimeZone, Zoned};

static TZ: LazyLock<TimeZone> = LazyLock::new(|| TimeZone::get("Asia/Tokyo").unwrap());

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Builder::from_env(Env::default().default_filter_or("info,html5ever=off"))
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

    let controller = GanRobotController::try_new().await?;
    controller.scramble(8).await?;
    controller.disconnect().await?;

    Ok(())
}
