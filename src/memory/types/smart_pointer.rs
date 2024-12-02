pub enum SmartPointerType {
    BoxPointer,
    RcPointer,
    ArcPointer,
}

pub struct SmartPointer {
    pub pointer_address: u64,
    pub pointee_address: u64,
    pub strong_refs: usize,
    pub weak_refs: usize,
    pub smart_pointer_type: SmartPointerType, // Renamed from `type`
}
