#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InputRecord {
    pub(crate) id: u32,

    pub(crate) price: f32,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OutputRecord {
    pub(crate) id: u32,

    pub(crate) price: f32,

    pub(crate) tax: f32,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Error {
    pub(crate) message: String,
}
