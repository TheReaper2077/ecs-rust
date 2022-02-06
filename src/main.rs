use std::{any::Any, marker::PhantomData};

pub(crate) mod archetype;
pub(crate) mod registry;
pub(crate) mod component;
pub(crate) mod entity;
pub mod system;

#[derive(Debug, Clone, Copy)]
struct Pos {
    v: u32,
}

#[derive(Debug)]
struct Vel {
    
}

fn main() {
    let mut registry = registry::Registry::new();
    let entity = registry.create_entity();

    let data = Pos {v:100};
    let v_data = Vel {};
    
    registry.register_component::<Pos>();
    registry.register_component::<Vel>();
    
    registry.add_component(entity, data);
    registry.add_component(entity, v_data);

    let x = registry.get_mut_component::<Pos>(entity);

    println!("{:?}", registry.view::<(Vel, Pos)>());
}
