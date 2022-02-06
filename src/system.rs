use std::{collections::{BTreeSet, HashSet, HashMap}, any::{TypeId, type_name}, hash, slice::SliceIndex};

use crate::{entity::{Entity, EntityManager}, component::ComponentManager};

pub struct SystemManager<'a> {
	system_entity_map: HashMap<&'a str, HashSet<Entity>>,
	system_component_map: HashMap<&'a str, HashSet<TypeId>>,
}

impl SystemManager<'_> {
	pub fn new() -> SystemManager<'static> {
		SystemManager {
			system_entity_map: HashMap::new(),
			system_component_map: HashMap::new(),
		}
	}

	pub fn init<T: 'static>(&mut self, component_manager: &ComponentManager, entity_manager: &mut EntityManager) {
		self.system_component_map.insert(type_name::<T>(), HashSet::new());
		self.system_entity_map.insert(type_name::<T>(), HashSet::new());
		
		for i in type_name::<T>().replace("(", "").replace(")", "").replace(",", "").split(" ") {
			println!("{} {:?}", i, component_manager.get_component_type_id(i));
			self.system_component_map.get_mut(type_name::<T>()).unwrap().insert(component_manager.get_component_type_id(i));
		}
		
		for pair in entity_manager.get_entity_component_map().iter() {
			if self.system_component_map.get(type_name::<T>()).unwrap().is_subset(pair.1) {
				self.system_entity_map.get_mut(type_name::<T>()).unwrap().insert(*pair.0);
			}
		}
	}

	pub fn add_component<T: 'static>(&mut self, entity: Entity, entity_manager: &mut EntityManager) {
		for pair in self.system_component_map.iter() {
			if pair.1.contains(&TypeId::of::<T>()) {
				if entity_manager.get_entity_component_map().get(&entity).unwrap().is_subset(pair.1) {
					self.system_entity_map.get_mut(pair.0).unwrap().insert(entity);
				}
			}
		}
	}

	pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
		for pair in self.system_component_map.iter() {
			if pair.1.contains(&TypeId::of::<T>()) {
				self.system_entity_map.get_mut(pair.0).unwrap().remove(&entity);
			}
		}
	}

	pub fn view<T: 'static>(&mut self, component_manager: &ComponentManager, entity_manager: &mut EntityManager) -> &HashSet<Entity> {
		if !self.system_entity_map.contains_key(type_name::<T>()) {
			self.init::<T>(component_manager, entity_manager);
		}

		self.system_entity_map.get(type_name::<T>()).unwrap()
	}

	pub fn destroy_entity(&mut self, entity: Entity) {
		for pair in self.system_entity_map.iter_mut() {
			pair.1.remove(&entity);
		}
	}
}