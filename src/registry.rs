use crate::{entity::{self, EntityManager, Entity}, component::ComponentManager};

pub struct Registry<'a> {
	entity_manager: EntityManager,
	component_manager: ComponentManager<'a>,
}


impl Registry<'_> {
	pub fn new() -> Registry<'static> {
		Registry {
			entity_manager: EntityManager::new(),
			component_manager: ComponentManager::new(),
		}
	}

	pub fn create_entity(&mut self) -> Entity {
		self.entity_manager.create_entity()
	}

	pub fn destroy_entity(&mut self, entity: Entity) {
		self.entity_manager.destroy_entity(entity)
	}

	pub fn register_component<T: 'static>(&mut self) {
		self.component_manager.register_component::<T>();
	}
	
	pub fn add_component<T: 'static>(&mut self, entity: Entity, data: T) {
		self.component_manager.add_component(entity, data);
	}

	pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
		self.component_manager.remove_component::<T>(entity);
	}

	pub fn get_mut_component<T: 'static>(&mut self, entity: Entity) -> &mut T{
		self.component_manager.get_mut_component::<T>(entity)
	}

	pub fn get_ref_component<T: 'static>(&mut self, entity: Entity) -> &T {
		self.component_manager.get_ref_component::<T>(entity)
	}
}