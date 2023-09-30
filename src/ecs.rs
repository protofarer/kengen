#[allow(warnings, dead_code)]
use crate::dsa::{BitSet, FixedSizeQueue};
use std::any::TypeId;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Index;
use std::rc::Rc;

mod components {
    pub mod transformcomponent;
}

mod systems {
    pub mod movementsystem;
}

#[derive(PartialEq, Eq, Hash)]
struct Entity {
    id: usize,
    // registry: Registry
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn kill() {}

    // pub fn add_component<TComponent>( ...args) {}
    // pub fn remove_component<TComponent>() {}
    // pub fn has_component<TComponent>() -> bool {}
    // pub fn get_component<TComponent>() -> TComponent {}
}

const MAX_COMPONENTS: u8 = 32;

type Signature = BitSet;

struct Component<T> {
    next_id: usize, // how to make static so shared across Component types
}

impl<T> Component<T> {}

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

trait IPool<T> {
    fn is_empty(&self) -> bool;
    fn add(&mut self, object: T);
    fn get_size(&self) -> usize;
    fn get(&self, index: usize) -> T;
    fn set(&self, index: usize, object: T);
}

// trait bound to component types
struct Pool<T> {
    data: Vec<T>,
    size: usize,
    entity_id_to_index: HashMap<u32, u32>,
    index_to_entity_id: HashMap<u32, u32>,
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
            entity_id_to_index: HashMap::new(),
            index_to_entity_id: HashMap::new(),
        }
    }
}

impl<T> IPool<T> for Pool<T> {
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn get_size(&self) -> usize {
        self.data.len()
    }
    fn add(&mut self, object: T) {
        self.data.push(object);
    }
    fn get(&self, index: usize) -> T {
        self.data[index]
    }
    fn set(&self, index: usize, object: T) {
        self.data[index] = object;
    }
}

impl<T> Index<usize> for Pool<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// TODO how to use Pool type without "caring" about Pool's generic?
pub struct Registry {
    n_entities: usize,
    component_pools: Vec<Box<dyn IPool>>,
    entity_component_signatures: Vec<Signature>,
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
            component_pools: Vec::new(),
            entity_component_signatures: Vec::new(),
            systems: HashMap::new(),
            entities_to_be_added: HashSet::new(),
            entities_to_be_killed: HashSet::new(),
            free_ids: VecDeque::new(),
        }
    }

    // * Entity Management
    // pub fn create_entity() -> Entity {}
    // pub fn kill_entity(entity: Entity) {}

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
