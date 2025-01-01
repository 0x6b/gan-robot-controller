mod face_rotation;
mod gan_robot_controller;

pub use face_rotation::{FaceRotation, FaceRotationMap};
pub use gan_robot_controller::GanRobotController;

pub const MAX_MOVES_PER_WRITE: usize = 36;
