use std::{collections::HashMap, io::Write, sync::LazyLock};

use btleplug::{
    api::{
        Central, CentralEvent, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
    },
    platform::{Adapter, Manager, Peripheral, PeripheralId},
};
use env_logger::{
    fmt::style::{AnsiColor, Style},
    Builder, Env,
};
use futures::StreamExt;
use jiff::{tz::TimeZone, Zoned};
use log::info;
use uuid::Uuid;

static TZ: LazyLock<TimeZone> = LazyLock::new(|| TimeZone::get("Asia/Tokyo").unwrap());
static MOVE_MAP: LazyLock<MoveMap> = LazyLock::new(MoveMap::new);

pub struct GanRobotController {
    gan_robot: Peripheral,
    move_characteristic: Characteristic,
}

impl GanRobotController {
    pub async fn try_new() -> anyhow::Result<Self> {
        let manager = Manager::new().await?;
        let central = Self::get_central(&manager).await;

        let mut events = central.events().await?;
        info!("Scanning for GAN robot");
        central.start_scan(ScanFilter::default()).await?;

        while let Some(event) = events.next().await {
            if let CentralEvent::DeviceDiscovered(id) = event {
                if let Some(gan_robot) = Self::find_gan_robot(&central, &id).await? {
                    gan_robot.connect().await?;
                    let move_characteristic = Self::find_move_characteristic(&gan_robot).await?;
                    return Ok(Self { gan_robot, move_characteristic });
                } else {
                    continue;
                }
            }
        }

        Err(anyhow::anyhow!("GAN robot not found"))
    }

    async fn do_move(&self, moves: &[u8]) -> anyhow::Result<()> {
        self.gan_robot
            .write(&self.move_characteristic, moves, WriteType::WithoutResponse)
            .await?;
        Ok(())
    }

    async fn disconnect(&self) -> anyhow::Result<()> {
        self.gan_robot.disconnect().await?;
        Ok(())
    }

    async fn get_central(manager: &Manager) -> Adapter {
        let adapters = manager.adapters().await.unwrap();
        adapters.into_iter().next().unwrap()
    }

    async fn find_gan_robot(
        central: &Adapter,
        id: &PeripheralId,
    ) -> anyhow::Result<Option<Peripheral>> {
        let peripheral = central.peripheral(id).await?;
        let properties = peripheral.properties().await?;
        let name = properties.and_then(|p| p.local_name).unwrap_or_default();
        if name == "GAN-a7f13" {
            central.stop_scan().await?;
            peripheral.connect().await?;
            info!("Connected: {id:?} {name}");
            return Ok(Some(peripheral));
        }
        Ok(None)
    }

    async fn find_move_characteristic(peripheral: &Peripheral) -> anyhow::Result<Characteristic> {
        peripheral.discover_services().await?;
        for service in peripheral.services() {
            for characteristic in service.characteristics {
                if characteristic.uuid == Uuid::parse_str("0000fff3-0000-1000-8000-00805f9b34fb")? {
                    return Ok(characteristic);
                }
            }
        }
        Err(anyhow::anyhow!("Move characteristic not found"))
    }
}

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
    let moves = MOVE_MAP.get_random_moves(8);
    controller.do_move(&moves).await?;
    controller.disconnect().await?;

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Move {
    R,
    R2,
    R2Prime,
    RPrime,
    F,
    F2,
    F2Prime,
    FPrime,
    D,
    D2,
    D2Prime,
    DPrime,
    L,
    L2,
    L2Prime,
    LPrime,
    B,
    B2,
    B2Prime,
    BPrime,
}

struct MoveMap {
    map: HashMap<Move, u8>,
}

impl MoveMap {
    fn new() -> Self {
        use Move::*;
        let mut map = HashMap::new();
        map.insert(R, 0);
        map.insert(R2, 1);
        map.insert(R2Prime, 1);
        map.insert(RPrime, 2);
        map.insert(F, 3);
        map.insert(F2, 4);
        map.insert(F2Prime, 4);
        map.insert(FPrime, 5);
        map.insert(D, 6);
        map.insert(D2, 7);
        map.insert(D2Prime, 7);
        map.insert(DPrime, 8);
        map.insert(L, 9);
        map.insert(L2, 10);
        map.insert(L2Prime, 10);
        map.insert(LPrime, 11);
        map.insert(B, 12);
        map.insert(B2, 13);
        map.insert(B2Prime, 13);
        map.insert(BPrime, 14);
        Self { map }
    }

    fn get(&self, mv: Move) -> u8 {
        *self.map.get(&mv).unwrap()
    }

    fn get_random_moves(&self, n: usize) -> Vec<u8> {
        use rand::{seq::SliceRandom, thread_rng};
        let moves = self.map.keys().cloned().collect::<Vec<_>>();
        moves
            .choose_multiple(&mut thread_rng(), n)
            .map(|m| self.get(*m))
            .collect()
    }
}
