
#[derive(Debug, Clone)]
pub enum CsdVariableType {
    String(String),
    Int(i32),
    Array(Vec<i32>),
    Float(f32)
}

#[derive(Debug, Clone)]
pub struct CsdVariable {
    name: String,
    value: CsdVariableType
}
