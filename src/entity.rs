use std::{collections::{HashMap, HashSet}, hash::Hash, any::{type_name, TypeId}, ops::IndexMut};

pub type Entity = usize;

pub struct EntityManager {
	next_entity: Entity,
	available_entities: Vec<Entity>,
	component_entity_map: HashMap<Entity, HashSet<TypeId>>,
}

impl EntityManager {
	pub fn new() -> EntityManager {
		EntityManager {
			next_entity: 0,
			available_entities: vec![],
			component_entity_map: HashMap::new(),
		}
	}

	pub fn create_entity(&mut self) -> Entity {
		let mut entity = self.next_entity;

		if self.available_entities.len() > 0 {
			entity = self.available_entities.pop().unwrap();
		} else {
			self.next_entity += 1;
		}

		self.component_entity_map.insert(entity, HashSet::new());
		entity
	}

	pub fn add_component<T: 'static>(&mut self, entity: Entity) {
		self.add_component_by_type_id(entity, &TypeId::of::<T>());
	}

	pub fn add_component_by_type_id(&mut self, entity: Entity, component_type: &TypeId) {
		self.component_entity_map.get_mut(&entity).unwrap().insert(*component_type);
	}

	pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
		self.component_entity_map.get_mut(&entity).unwrap().remove(&TypeId::of::<T>());
	}

	pub fn get_entity_component_map(&mut self) -> &mut HashMap<Entity, HashSet<TypeId>> {
		&mut self.component_entity_map
	}

	pub fn get_entity_components(&mut self, entity: Entity) -> &HashSet<TypeId> {
		self.component_entity_map.get(&entity).unwrap()
	}

	pub fn destroy_entity(&mut self, entity: Entity) {
		self.available_entities.push(entity);
		self.component_entity_map.remove(&entity);
	}
}