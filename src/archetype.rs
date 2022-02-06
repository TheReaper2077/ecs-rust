use std::any::{TypeId, Any};

use crate::entity::Entity;

struct Archetype {
	entities_list: Vec<Entity>,
	components_list: Vec<TypeId>
}

impl Archetype {
	pub fn new() -> Archetype {
		Archetype {
			entities_list: vec![],
			components_list: vec![],
		}
	}

	pub fn register_components<T: 'static>() {

	}
}