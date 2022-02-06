use std::{any::Any, marker::PhantomData};

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
    
    registry.register_component::<Pos>();
    registry.register_component::<Vel>();
    
    registry.add_component(entity, Pos{v:0});
    registry.add_component(entity, Vel{});

    let x = registry.get_mut_component::<Pos>(entity);
    
    let a_entity = registry.create_entity();
    
    registry.add_component(a_entity, Pos{v:110});
    registry.add_component(a_entity, Vel{});    
    
    println!("{:?}", registry.view::<(Vel, Pos)>());
    
    registry.remove_component::<Pos>(entity);

    println!("{:?}", registry.view::<(Pos, Vel)>());
}
