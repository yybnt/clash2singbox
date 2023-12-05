use serde::{Serialize, Deserialize};

use super::types::TunCommon;


#[derive(Debug, Serialize, Deserialize)]
pub struct TunMode {
    pub enable: bool,
    #[serde(flatten)]
    pub tun: TunCommon,
}