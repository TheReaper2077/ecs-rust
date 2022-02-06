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
	
	registry.register_component::<Pos>();
	registry.register_component::<Vel>();
	
	// registry.create_entity::<(Pos, Vel)>();

	println!("{:?}", registry.view::<Vel>());
}
