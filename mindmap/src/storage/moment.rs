use std::{collections::HashMap};
use chrono::{DateTime, Utc};
use uuid::{Uuid};
use crate::types::types::PayloadTypes;

#[derive(Debug, Clone)]
pub struct Moment <const DIM: usize> {
    pub uuid: uuid::Uuid,
    pub chain_id: Option<uuid::Uuid>,
    pub vector: HashMap<String, [f32; DIM]>,
    pub timestamp: DateTime<Utc>,
    pub payload: HashMap<String, PayloadTypes>,
    pub related_moments: Option<Vec<Uuid>>,
}

 #[derive(Debug, Clone, Default)]
pub struct MomentBuilder <const DIM: usize>{
    uuid: Option<Uuid>,
    chain_id: Option<Uuid>,
    vector: HashMap<String, [f32; DIM]>,
    timestamp: DateTime<Utc>,
    payload: HashMap<String, PayloadTypes>,
    related_moments: Option<Vec<Uuid>>,
}

impl <const DIM: usize> MomentBuilder <DIM> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            chain_id: None,
            vector: HashMap::new(),
            timestamp: chrono::Utc::now(),
            payload: HashMap::new(),
            related_moments: None,
        }
    }
    
    pub fn with_uuid(mut self, uuid: Uuid) -> Self {
        self.uuid = Some(uuid);
        self
    }

    pub fn with_payload(mut self, payload: HashMap<String, PayloadTypes>) -> Self {
        self.payload = payload;
        self
    }

    pub fn with_relations(mut self, relations: Vec<Uuid>) -> Self {
        self.related_moments = Some(relations);
        self
    }

    pub fn with_chain(mut self, chain_id: Uuid) -> Self {
        self.chain_id = Some(chain_id);
        self
    }

    pub fn with_time(mut self, time: DateTime<Utc>) -> Self {
        self.timestamp = time;
        self
    }
    
    pub fn with_vector(mut self, key: String, vector: [f32; DIM]) -> Self {
        self.vector.insert(key, vector);
        self
    }

    pub fn build(self) -> Result<Moment<DIM>, String> {
        if self.vector.is_empty() {
            return Err("Vector is empty".to_string())
        }

        Ok(Moment {
            uuid: self.uuid.unwrap_or(Uuid::now_v7()),
            chain_id: self.chain_id,
            vector: self.vector,
            timestamp: self.timestamp,
            payload: self.payload,
            related_moments: self.related_moments,
        })
    }
}

// TODO: add implimentation methods for Moment