mod dsa;
use dsa::BitSet;

pub struct Entity {
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
    pub fn has_component<TComponent>() -> bool {}
    pub fn get_component<TComponent>() -> TComponent {}
}

const MAX_COMPONENTS: u8 = 32;

type Signature = BitSet;

struct System {
    componentSignature: Signature,
}

impl System {
    pub fn new() -> Self {
        Self {
            componentSignature: BitSet::new(),
        }
    }
}
