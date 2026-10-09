use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub const MAGIC: [u8; 4] = *b"MNDB";
pub const CURRENT_SHARD_VERSION: u16 = 1;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct ShardHeader {
    pub magic: [u8; 4],
    pub shard_format_version: u16,
    pub shard_kind: ShardKind,
    pub shard_id: u32,

    pub record_format: RecordFormat,
    pub compression: Compression,
    pub endianness: Endianness,

    pub created_at: u64,
    pub sealed_at: Option<u64>,     // if None than we can add records, else we cannot
    pub record_count: u64,
    pub data_bytes: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecordHeader {
    pub record_version: u8,
    pub record_type: RecordType,
    pub payload_len: u32,
    pub checksum: u32
}

//TODO: implement