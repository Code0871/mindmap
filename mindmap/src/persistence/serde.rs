use std::{Serialize, Desirialize};
use crate::types::types::{Moment, Collection};
use uuid::Uuid;

#[derive(Serialize, Desirialize)]
pub struct ShardHeader {
    pub magic: [u8; 4], // Magic number for the shards file
    pub shard_format_version: u16 // Version format

}