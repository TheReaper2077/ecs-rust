use std::{any::Any, marker::PhantomData};

pub(crate) mod system;
pub(crate) mod registry;
pub(crate) mod component;
pub(crate) mod entity;

struct Pos {
    v: u32,
}

struct Vel {
    
}

fn main() {
    let mut registry = registry::Registry::new();
    let entity = registry.create_entity();

    let data = Pos {v:100};
    
    registry.register_component::<Pos>();
    registry.register_component::<Vel>();
    
    registry.add_component(entity, data);
}
