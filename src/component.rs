use std::collections::HashMap;
use std::any::{TypeId, Any, type_name};
use std::ops::DerefMut;

use crate::entity::{Entity, self};

pub struct ComponentArray<T: 'static> {
	component_store: Vec<T>,
	entity_index_map: HashMap<Entity, usize>,
	index_entity_map: HashMap<usize, Entity>,
	next_index: usize,
}

impl<T: 'static> ComponentArray<T> {
	pub fn new() -> ComponentArray<T> {
		ComponentArray::<T> {
    		component_store: vec![],
			next_index: 0,
    		entity_index_map: HashMap::new(),
			index_entity_map: HashMap::new(),
		}
	}

	pub fn add(&mut self, entity: Entity, data: T) {
		self.component_store.push(data);
		self.entity_index_map.insert(entity, self.next_index);
		self.index_entity_map.insert(self.next_index, entity);
		self.next_index += 1;
	}

	pub fn remove(&mut self, entity: Entity) {
		self.next_index -= 1;
		// self.component_store.swap_remove(self.entity_index_map[&entity]);
		
		let removed_index = self.entity_index_map[&entity];

		self.index_entity_map.insert(removed_index, self.index_entity_map[&self.next_index]);
		self.entity_index_map.insert(entity, self.next_index);
		self.component_store.swap_remove(removed_index);
	}
	
	pub fn get_mut(&mut self, entity: Entity) -> &mut T {
		self.component_store.get_mut(self.entity_index_map[&entity]).unwrap()
	}

	pub fn get_ref(&mut self, entity: Entity) -> &T {
		self.component_store.get(self.entity_index_map[&entity]).unwrap()
	}
}

trait IComponentArray {
	fn as_any_mut(&mut self) -> &mut dyn Any;
	fn as_any_ref(&self) -> &dyn Any;
	fn destroy_entity(&mut self, entity: Entity);
}

impl<T: 'static> IComponentArray for ComponentArray<T> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn destroy_entity(&mut self, entity: Entity) {
        if (self.entity_index_map.contains_key(&entity)) {
			self.remove(entity);
		}
    }
}

pub struct ComponentManager<'a> {
	component_array: Vec<Box<dyn IComponentArray>>,
	component_array_index_map: HashMap<TypeId, usize>,
	component_name_id_map: HashMap<&'a str, TypeId>,
	next_component_array_index: usize,
}

impl ComponentManager<'_> {
	pub fn new() -> ComponentManager<'static> {
		ComponentManager {
			component_array: vec![],
			component_array_index_map: HashMap::new(),
			next_component_array_index: 0,
			component_name_id_map: HashMap::new(),
		}
	}

	pub fn register_component<T: 'static>(&mut self) {
		assert!(!self.component_array_index_map.contains_key(&TypeId::of::<T>()), "[COMPONENT] {} is already registered", type_name::<T>());

		self.component_array.push(Box::new(ComponentArray::<T>::new()));
		self.component_array_index_map.insert(TypeId::of::<T>(), self.next_component_array_index);
		self.component_name_id_map.insert(type_name::<T>(), TypeId::of::<T>());
		self.next_component_array_index += 1;
	}

	pub fn add_component<T: 'static>(&mut self, entity: Entity, data: T) {
		assert!(self.component_array_index_map.contains_key(&TypeId::of::<T>()), "[COMPONENT] {} is not registered", type_name::<T>());

		self.component_array.get_mut(self.component_array_index_map[&TypeId::of::<T>()]).unwrap().as_any_mut().downcast_mut::<ComponentArray<T>>().unwrap().add(entity, data);
	}

	pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
		assert!(self.component_array_index_map.contains_key(&TypeId::of::<T>()), "[COMPONENT] {} is not present / it was previously removed", type_name::<T>());
		
		self.component_array.get_mut(self.component_array_index_map[&TypeId::of::<T>()]).unwrap().as_any_mut().downcast_mut::<ComponentArray<T>>().unwrap().remove(entity);
	}

	pub fn get_mut_component<T: 'static>(&mut self, entity: Entity) -> &mut T {
		assert!(self.component_array_index_map.contains_key(&TypeId::of::<T>()), "[COMPONENT] {} is not registered", type_name::<T>());
		
		self.component_array.get_mut(self.component_array_index_map[&TypeId::of::<T>()]).unwrap().as_any_mut().downcast_mut::<ComponentArray<T>>().unwrap().get_mut(entity)
	}

	pub fn get_ref_component<T: 'static>(&mut self, entity: Entity) -> &T {
		assert!(self.component_array_index_map.contains_key(&TypeId::of::<T>()), "[COMPONENT] {} is not registered", type_name::<T>());

		self.component_array.get_mut(self.component_array_index_map[&TypeId::of::<T>()]).unwrap().as_any_mut().downcast_mut::<ComponentArray<T>>().unwrap().get_ref(entity)
	}

	pub fn get_component_type_id(&self, component_name: &str) -> TypeId {
		assert!(self.component_name_id_map.contains_key(component_name), "[COMPONENT] {} is not registered", component_name);

		self.component_name_id_map[component_name]
	}

	pub fn destroy_entity(&mut self, entity: Entity) {
		for array in self.component_array.iter_mut() {
			array.destroy_entity(entity);
		}
	}
}

