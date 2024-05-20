use std::sync::atomic::AtomicU16;

pub static COUNTER: AtomicU16 = AtomicU16::new(0);

pub trait MyTypeId {
    fn get_type_id(&self) -> u16;
}

pub trait MyTypeIdStatic {
    fn get_type_id_static() -> u16;
}
