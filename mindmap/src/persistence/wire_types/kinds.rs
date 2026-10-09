use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ShardKind {
    Moments = 0,
    Chains = 1,
    Mixed = 2,
    Index = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RecordType {
    Moment = 0,
    Chain = 1,
}