use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::BTreeMap;
use crate::types::types::Moment;

pub const CURRENT_MOMENT_VERSION: u8 = 1;
pub const CURRENT_PAYLOAD_VERSION: u8 = 1;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PayloadOnDisk {
    Text(String),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Bool(bool),
    Array(Vec<PayloadOnDisk>),
    Map(BTreeMap<String, PayloadOnDisk>),
    Json(Vec<u8>),
    Null,
}

#[derive(Serialize, Deserialize)]
pub struct MomentOnDisk {
    pub version: u8,                                // version of moment struct
    #[serde(with = "uuid::serde::compact")]
    pub uuid: Uuid,                                 // uuid of Moment
    #[serde(with = "uuid::serde::compact")]
    pub chain_uuid: Option<Uuid>,                   // uuid of chain where moment can be
    pub vector: BTreeMap<String, Vec<f32>>,
    pub timestamp_millis: u64,                      // timestamp in milliseconds
    pub payload: BTreeMap<String, PayloadOnDisk>,   // payload
    #[serde(with = "uuid::serde::compact")]
    pub related_moments: Option<Vec<Uuid>>          // moments that are can be connected to this one
}

// TODO: implement convert from domain to wire