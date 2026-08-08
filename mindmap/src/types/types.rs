use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum VectorTypes {
    Dense128([f32; 128]),
    Dense256([f32; 256]),
    Dense768([f32; 768]),
    Sparse(HashMap<usize, f32>),
    Custom(Vec<f32>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PayloadTypes{
    Text(String),
    Int32(i32),
    Int64(i64),
    Float(f32),
    Float64(f64),
    Bool(bool),
    Array(Vec<PayloadTypes>),
    HashMap(HashMap<String, PayloadTypes>),
    Null,
}