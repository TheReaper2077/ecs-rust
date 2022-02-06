use std::{collections::{BTreeSet, HashSet, HashMap}, any::{TypeId, type_name}, hash, slice::SliceIndex, ops::Sub};

use crate::{entity::{Entity, EntityManager}, component::ComponentManager};

pub struct SystemManager<'a> {
	system_entities: Vec<HashSet<Entity>>,
	system_components: Vec<HashSet<TypeId>>,
	registered_systems: HashMap<&'a str, usize>,
	next_index: usize,
}

impl SystemManager<'_> {
	pub fn new() -> SystemManager<'static> {
		SystemManager {
			system_entities: vec![],
			system_components: vec![],
			registered_systems: HashMap::new(),
			next_index: 0,
		}
	}

	pub fn init<T: 'static>(&mut self, component_manager: &ComponentManager, entity_manager: &mut EntityManager) {
		let mut component_set = HashSet::<TypeId>::new();
		
		for i in type_name::<T>().replace("(", "").replace(")", "").replace(",", "").split(" ") {
			component_set.insert(component_manager.get_component_type_id(i));
		}

		// check for duplicates
		for i in 0..self.system_components.len() {
			if self.system_components[i].eq(&component_set) {
				self.registered_systems.insert(type_name::<T>(), i);
				return;
			}
		}

		let mut entity_set = HashSet::<Entity>::new();
		
		for pair in entity_manager.get_entity_component_map().iter() {
			if component_set.is_subset(&pair.1) {
				entity_set.insert(*pair.0);
			}
		}
		
		self.system_components.push(component_set);
		self.system_entities.push(entity_set);
		self.registered_systems.insert(type_name::<T>(), self.system_components.len() - 1);
	}
	
	pub fn entity_changed<T: 'static>(&mut self, entity: Entity, entity_components: &HashSet<TypeId>) {
		// for i in 0..self.system_components.len() {
		// 	if self.system_components[i].contains(&TypeId::of::<T>()) {
		// 		if self.system_components[i].is_subset(entity_components) {
		// 			self.system_entities[i].insert(entity);
		// 		} else {
		// 			self.system_entities[i].remove(&entity);
		// 		}
		// 	}
		// }
		self.entity_changed_by_type_id(entity, entity_components, &TypeId::of::<T>());
	}

	pub fn entity_changed_by_type_id(&mut self, entity: Entity, entity_components: &HashSet<TypeId>, component_type: &TypeId) {
		for i in 0..self.system_components.len() {
			if self.system_components[i].contains(component_type) {
				if self.system_components[i].is_subset(entity_components) {
					self.system_entities[i].insert(entity);
				} else {
					self.system_entities[i].remove(&entity);
				}
			}
		}
	}

	pub fn view<T: 'static>(&mut self, component_manager: &ComponentManager, entity_manager: &mut EntityManager) -> &HashSet<Entity> {
		if !self.registered_systems.contains_key(type_name::<T>()) {
			self.init::<T>(component_manager, entity_manager);
		}

		&self.system_entities[*self.registered_systems.get(type_name::<T>()).unwrap()]
	}

	pub fn destroy_entity(&mut self, entity: Entity) {
		for set in self.system_entities.iter_mut() {
			set.remove(&entity);
		}
	}
}