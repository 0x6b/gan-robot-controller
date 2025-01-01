use std::{
    io::{stdin, Write},
    sync::LazyLock,
};

use clap::Parser;
use env_logger::{
    fmt::style::{AnsiColor, Style},
    Builder, Env,
};
use jiff::{tz::TimeZone, Zoned};
use lib::{FaceRotation, GanRobotController, MAX_MOVES_PER_WRITE};
use log::info;

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

    /// The status characteristic UUID of the GAN robot.
    #[arg(
        short,
        long,
        env = "GAN_ROBOT_STATUS_CHARACTERISTIC",
        default_value = "0000fff2-0000-1000-8000-00805f9b34fb"
    )]
    pub status_characteristic: String,

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
        /// The move sequence to do on the cube. Each move should be separated by whitespace.
        /// Please note that the moves should be in the format of face rotation strings. Valid
        /// strings include `R`, `R'`, `R2`, `R2'`, `F`, `F'`, `F2`, `F2'`, `D`, `D'`, `D2`, `D2'`,
        /// `L`, `L'`, `L2`, `L2'`, `B`, `B'`, `B2`, `B2'`.
        moves: String,
    },

    /// Enter a REPL to interact with the cube.
    Repl {
        /// Use raw u8 values for moves instead of the default face rotation strings like "R",
        /// "R2", "R'".
        #[arg(short, long)]
        debug: bool,
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

    let Args {
        name,
        move_characteristic,
        status_characteristic,
        command,
    } = Args::parse();
    let controller =
        GanRobotController::try_new(&name, &move_characteristic, &status_characteristic)?
            .try_connect()
            .await?;

    match command {
        Command::Scramble { num } => {
            if num > MAX_MOVES_PER_WRITE {
                anyhow::bail!(
                    "Too many moves: {num}. Can only scramble with {MAX_MOVES_PER_WRITE} moves at a time"
                );
            }
            controller.scramble(num).await?
        }
        Command::Move { moves } => {
            controller
                .do_moves(&moves.split_whitespace().map(FaceRotation::from).collect::<Vec<_>>())
                .await?
        }
        Command::Repl { debug } => {
            info!("Entering REPL. Type `exit` to exit.");
            loop {
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                let input = input.trim();

                if input == "exit" {
                    break;
                }

                if debug {
                    let moves = input
                        .split_whitespace()
                        .map(|s| s.parse::<u8>().unwrap_or_default())
                        .collect::<Vec<_>>();
                    controller.do_moves_raw(&moves).await?;
                } else {
                    controller
                        .do_moves(
                            &input.split_whitespace().map(FaceRotation::from).collect::<Vec<_>>(),
                        )
                        .await?;
                }
            }
        }
    }

    controller.disconnect().await?;

    Ok(())
}
