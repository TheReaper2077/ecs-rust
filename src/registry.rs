use std::{hash, collections::{BTreeSet, HashSet}, any::type_name};

use crate::{entity::{self, EntityManager, Entity}, component::ComponentManager, system::SystemManager};

pub struct Registry<'a> {
	entity_manager: EntityManager,
	component_manager: ComponentManager<'a>,
	system_manager: SystemManager<'a>,
}


trait OverLoading {
	fn create_entity<T: 'static>(&mut self) -> Entity;
}


impl Registry<'_> {
	pub fn new() -> Registry<'static> {
		Registry {
			entity_manager: EntityManager::new(),
			component_manager: ComponentManager::new(),
			system_manager: SystemManager::new(),
		}
	}

	pub fn create_entity(&mut self) -> Entity {
		self.entity_manager.create_entity()
	}

	pub fn create_entity_explicit<T: 'static>(&mut self) -> Entity {
	    let entity = self.entity_manager.create_entity();

		for i in type_name::<T>().replace("(", "").replace(")", "").replace(",", "").split(" ") {
			let component_type = self.component_manager.get_component_type_id(i);
			self.entity_manager.add_component_by_type_id(entity, &component_type);
			self.system_manager.entity_changed_by_type_id(entity, self.entity_manager.get_entity_components(entity), &component_type);
		}

		entity
	}

	pub fn destroy_entity(&mut self, entity: Entity) {
		self.entity_manager.destroy_entity(entity);
		self.component_manager.destroy_entity(entity);
		self.system_manager.destroy_entity(entity);
	}

	pub fn register_component<T: 'static>(&mut self) {
		self.component_manager.register_component::<T>();
	}

	pub fn add_component<T: 'static>(&mut self, entity: Entity, data: T) {
		self.entity_manager.add_component::<T>(entity);
		self.component_manager.add_component::<T>(entity, data);
		self.system_manager.entity_changed::<T>(entity, self.entity_manager.get_entity_components(entity));
	}

	pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
		self.entity_manager.remove_component::<T>(entity);
		self.component_manager.remove_component::<T>(entity);
		self.system_manager.entity_changed::<T>(entity, self.entity_manager.get_entity_components(entity));
	}

	pub fn get_mut_component<T: 'static>(&mut self, entity: Entity) -> &mut T{
		self.component_manager.get_mut_component::<T>(entity)
	}

	pub fn get_ref_component<T: 'static>(&mut self, entity: Entity) -> &T {
		self.component_manager.get_ref_component::<T>(entity)
	}

	pub fn view<T: 'static>(&mut self) ->  &HashSet<Entity> {
		self.system_manager.view::<T>(&self.component_manager, &mut self.entity_manager)
	}
}