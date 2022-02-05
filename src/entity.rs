use std::{collections::HashMap, hash::Hash, any::type_name, ops::IndexMut};

pub type Entity = usize;

pub struct EntityManager {
	next_entity: Entity,
	available_entities: Vec<Entity>,
}

impl EntityManager {
	pub fn new() -> EntityManager {
		EntityManager {
			next_entity: 0,
			available_entities: vec![],
		}
	}

	pub fn create_entity(&mut self) -> Entity {
		let mut entity = self.next_entity;

		if self.available_entities.len() > 0 {
			entity = self.available_entities.pop().unwrap();
		} else {
			self.next_entity += 1;
		}

		entity
	}

	pub fn destroy_entity(&mut self, entity: Entity) {
		self.available_entities.push(entity);
	}
}