use std::{io::Write, sync::LazyLock};

use clap::Parser;
use env_logger::{
    fmt::style::{AnsiColor, Style},
    Builder, Env,
};
use jiff::{tz::TimeZone, Zoned};
use lib::{GanRobotController, Move};

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

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Scramble the cube with the given number of moves.
    Scramble {
        /// The number of moves to scramble the cube with.
        #[arg(short, long, default_value = "8")]
        num: usize,
    },

    /// Do moves on the cube with the given move sequence.
    Move {
        /// The move sequence to do on the cube.
        moves: String,
    },
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

    let Args { name, move_characteristic, command } = Args::parse();
    let controller = GanRobotController::try_new(&name, &move_characteristic)?
        .try_connect()
        .await?;

    match command {
        Command::Scramble { num } => controller.scramble(num).await?,
        Command::Move { moves } => {
            controller
                .do_moves(&moves.split_whitespace().map(Move::from).collect::<Vec<_>>())
                .await?
        }
    }

    controller.disconnect().await?;

    Ok(())
}
