use std::ops::Deref;

use btleplug::{
    api::{
        Central, CentralEvent, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
    },
    platform::{Adapter, Manager, Peripheral, PeripheralId},
};
use futures::StreamExt;
use log::info;
use uuid::Uuid;

use crate::{FaceRotation, FaceRotationMap};

pub trait State {}

pub struct Uninitialized {
    name: String,
    move_characteristic: Uuid,
}

impl State for Uninitialized {}

pub struct Connected {
    gan_robot: Peripheral,
    move_characteristic: Characteristic,
    face_rotation_map: FaceRotationMap,
}

impl State for Connected {}

pub struct GanRobotController<S>
where
    S: State,
{
    state: S,
}

impl<S> Deref for GanRobotController<S>
where
    S: State,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl GanRobotController<Connected> {
    pub async fn scramble(&self, num_moves: usize) -> anyhow::Result<()> {
        info!("Scrambling with {num_moves} moves");
        let moves = self.face_rotation_map.get_random_moves(num_moves);
        self.do_moves(&moves).await?;
        Ok(())
    }

    pub async fn do_moves(&self, moves: &[FaceRotation]) -> anyhow::Result<()> {
        info!(
            "Doing moves: {}",
            moves.iter().map(|m| m.to_string()).collect::<Vec<String>>().join(" ")
        );
        let moves = moves
            .iter()
            .filter(|m| **m != FaceRotation::Invalid)
            .map(u8::from)
            .collect::<Vec<u8>>();
        self.gan_robot
            .write(&self.move_characteristic, &moves, WriteType::WithoutResponse)
            .await?;
        Ok(())
    }

    pub async fn disconnect(&self) -> anyhow::Result<()> {
        info!("Disconnecting from GAN robot");
        self.gan_robot.disconnect().await?;
        Ok(())
    }
}

impl GanRobotController<Uninitialized> {
    pub fn try_new(name: &str, move_characteristic: &str) -> anyhow::Result<Self> {
        let name = name.to_string();
        let move_characteristic = Uuid::parse_str(move_characteristic)?;
        Ok(Self { state: Uninitialized { name, move_characteristic } })
    }

    pub async fn try_connect(self) -> anyhow::Result<GanRobotController<Connected>> {
        let manager = Manager::new().await?;
        let central = Self::get_central(&manager).await;

        let mut events = central.events().await?;
        info!("Scanning for GAN robot");
        central.start_scan(ScanFilter::default()).await?;

        while let Some(event) = events.next().await {
            if let CentralEvent::DeviceDiscovered(id) = event {
                if let Some(gan_robot) = Self::find_gan_robot(&central, &id, &self.name).await? {
                    gan_robot.connect().await?;
                    let move_characteristic =
                        Self::find_move_characteristic(&gan_robot, &self.move_characteristic)
                            .await?;
                    return Ok(GanRobotController {
                        state: Connected {
                            gan_robot,
                            move_characteristic,
                            face_rotation_map: FaceRotationMap::new(),
                        },
                    });
                } else {
                    continue;
                }
            }
        }

        Err(anyhow::anyhow!("GAN robot not found"))
    }

    async fn get_central(manager: &Manager) -> Adapter {
        let adapters = manager.adapters().await.unwrap();
        adapters.into_iter().next().unwrap()
    }

    async fn find_gan_robot(
        central: &Adapter,
        id: &PeripheralId,
        name: &str,
    ) -> anyhow::Result<Option<Peripheral>> {
        let peripheral = central.peripheral(id).await?;
        let properties = peripheral.properties().await?;
        let local_name = properties.and_then(|p| p.local_name).unwrap_or_default();
        if local_name == name {
            central.stop_scan().await?;
            peripheral.connect().await?;
            info!("Connected: {id:?} {name}");
            return Ok(Some(peripheral));
        }
        Ok(None)
    }

    async fn find_move_characteristic(
        peripheral: &Peripheral,
        uuid: &Uuid,
    ) -> anyhow::Result<Characteristic> {
        peripheral.discover_services().await?;
        for service in peripheral.services() {
            for characteristic in service.characteristics {
                if characteristic.uuid == *uuid {
                    return Ok(characteristic);
                }
            }
        }
        Err(anyhow::anyhow!("Move characteristic not found"))
    }
}
