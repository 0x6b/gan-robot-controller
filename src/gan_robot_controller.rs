use btleplug::{
    api::{
        Central, CentralEvent, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
    },
    platform::{Adapter, Manager, Peripheral, PeripheralId},
};
use futures::StreamExt;
use log::info;
use uuid::Uuid;

use crate::MoveMap;

pub struct GanRobotController {
    gan_robot: Peripheral,
    move_characteristic: Characteristic,
    move_map: MoveMap,
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
                    return Ok(Self {
                        gan_robot,
                        move_characteristic,
                        move_map: MoveMap::new(),
                    });
                } else {
                    continue;
                }
            }
        }

        Err(anyhow::anyhow!("GAN robot not found"))
    }

    pub async fn scramble(&self, num_moves: usize) -> anyhow::Result<Vec<u8>> {
        let moves = self.move_map.get_random_moves(num_moves);
        self.do_move(&moves).await?;
        Ok(moves)
    }

    pub async fn do_move(&self, moves: &[u8]) -> anyhow::Result<()> {
        self.gan_robot
            .write(&self.move_characteristic, moves, WriteType::WithoutResponse)
            .await?;
        Ok(())
    }

    pub async fn disconnect(&self) -> anyhow::Result<()> {
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
