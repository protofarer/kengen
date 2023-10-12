#[allow(warnings, dead_code)]
use crate::dsa::{BitSet, FixedSizeQueue};
use std::any::TypeId;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Index, IndexMut};
use std::rc::Rc;

pub mod components;
pub mod systems;

#[derive(PartialEq, Eq, Hash)]
pub struct Entity {
    id: usize,
}

// does not access registry (as does the C++ engine)
impl Entity {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    // pub fn add_component<TComponent>( ...args) {}
    // pub fn remove_component<TComponent>() {}
    // pub fn has_component<TComponent>() -> bool {}
    // pub fn get_component<TComponent>() -> TComponent {}
}

// trait bound to component types
struct Pool<TComponent> {
    components: Vec<TComponent>,
    entity_id_to_index: HashMap<u32, usize>,
    index_to_entity_id: HashMap<usize, u32>,
}

impl<TComponent> Pool<TComponent> {
    // use .reserve() ?
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            entity_id_to_index: HashMap::new(),
            index_to_entity_id: HashMap::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
    pub fn get_size(&self) -> usize {
        self.components.len()
    }
    pub fn add(&mut self, component: TComponent) {
        self.components.push(component);
    }
    pub fn get(&self, index: usize) -> &TComponent {
        &self.components[index]
    }
    pub fn set(&mut self, index: usize, component: TComponent) {
        self.components[index] = component;
    }
}

impl<TComponent> Index<usize> for Pool<TComponent> {
    type Output = TComponent;

    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

impl<TComponent> IndexMut<usize> for Pool<TComponent> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.components[index]
    }
}

const MAX_COMPONENTS: u8 = 32;

type Signature = BitSet;

// TODO make this a Trait
pub struct System {
    component_signature: Signature,
    entities: Vec<Entity>,
}

impl System {
    pub fn new() -> Self {
        Self {
            component_signature: BitSet::new(),
            entities: Vec::new(),
        }
    }
    // pub add_entity_to_system(entity: Entity) {}
    // pub remove_entity_from_system(entity: Entity) {}
    // pub get_system_entities() -> Vec<Entity> {}
    // pub require_component() {}
}

// TODO how to use Pool type without "caring" about Pool's generic?
pub struct Registry {
    n_entities: usize,
    // component_pools: Vec<Box<Pool>>,
    entity_component_signatures: Vec<Signature>,
    // change to Arc from Rc due to new registry singleton via lazy_static, for thread-safe atomic operations, do I need this, how do I know when this game engine is using multiple threads, how do I design for it... for later.. just get it working on 1 thread
    systems: HashMap<TypeId, Rc<System>>,  
    entities_to_be_added: HashSet<Entity>,
    entities_to_be_killed: HashSet<Entity>,

    //
    // entity_per_tag: HashMap<String, Entity>
    // tag_per_entity: HashMap<usize, String>

    // entities_per_group: HashMap<String, HashSet<Entity>>
    // group_per_entity: HashMap<usize, String>
    free_ids: VecDeque<usize>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            n_entities: 0,
            // component_pools: Vec::new(),
            entity_component_signatures: Vec::new(),
            systems: HashMap::new(),
            entities_to_be_added: HashSet::new(),
            entities_to_be_killed: HashSet::new(),
            free_ids: VecDeque::new(),
        }
    }

    // * Entity Management
    pub fn create_entity(&mut self) -> Entity {
        // check free entity vecdeque
        let entity_id: usize;
        if self.free_ids.is_empty() {
            self.n_entities += 1;
            entity_id = self.n_entities;

            // ? investigate vector resizing
            // if entity_id >= self.entity_component_signatures.len() {
            //     self.entity_component_signatures.reserve(self.entity_component_signatures.len() + 10);
            // }
        } else {
            entity_id = self.free_ids.pop_front().unwrap();
        }

        Entity::new(entity_id)
        // entity.registry = this;  // entity must use get_instance, eg registry::KillEntity, TagEntity, GroupEntity, HasTag, HasGroup
    }

    pub fn kill_entity(&mut self, entity: Entity) {
        self.entities_to_be_killed.insert(entity);
    }

    // * Component Management
    // pub fn add_component<TComponent, Targs>(entity: Entity, args: Targs) {}
    // pub fn remove_component<TComponent>(entity: Entity) {}
    // pub fn has_component<TComponent>(entity: Entity) -> bool {}
    // pub fn get_component<Tcomponent>(entity: Entity) -> TComponent {}

    // * System Management
    // TODO pub fn add_system<TSystem, TArgs>(args: TArgs) {}
    // pub fn remove_system<TSystem>() {}
    // pub fn has_system<TSystem>() -> bool {}
    // pub fn get_system<TSystem>() -> TSystem {}

    // * System-Entity Management
    // pub fn add_entity_to_systems(entity: Entity) {}
    // pub fn remove_entity_from_systems(entity: Entity) {}

    // * Tag Management
    // pub fn tag_entity(entity: Entity, tag: &str) {}
    // pub fn entity_has_tag(entity: Entity, tag: &str) -> bool {}
    // pub fn get_entity_by_tag(tag: &str) -> Entity {}
    // pub fn remove_entity_tag(entity: Entity) {}

    // * Group Management
    // pub fn group_entity(entity: Entity, group: &str) {}
    // pub fn entity_belongs_to_group(entity: Entity, group: &str) -> bool {}
    // pub fn get_entities_by_group(group: &str) -> Vec<Entity> {}
    // pub fn remove_entity_group(entity: Entity) {}
}
