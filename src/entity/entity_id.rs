extern crate proc_macro;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Entity(pub u64);

pub trait EntityId {
    fn id(&self) -> Entity;
}
