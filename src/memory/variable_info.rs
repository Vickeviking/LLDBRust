pub struct VariableInfo {
    pub name: String,
    pub address: u64,
    pub mutable: bool,
    pub moved: bool,
    pub frame_id: Option<u64>,
}
