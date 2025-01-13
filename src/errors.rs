

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrafficError {
    #[error("Index out of bounds")]
    IndexOutOfBounds,
    #[error("Invalid probability value")]
    InvalidProbability,
    #[error("Maximum view level reached")]
    MaxViewLevelReached,
    #[error("Invalid road connection")]
    InvalidRoadConnection,
}

pub type Result<T> = std::result::Result<T, TrafficError>;
