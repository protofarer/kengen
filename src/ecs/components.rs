use nalgebra::Vector2;
use std::sync::atomic::{AtomicU32, Ordering};

static COMPONENT_TYPE_ID: AtomicU32 = AtomicU32::new(1);

pub trait Component {
    fn type_id() -> u32;
    fn get_id() -> u32;
}

macro_rules! impl_component {
    ($type:ty) => {
        impl Component for $type {
            fn type_id() -> u32 {
                static TYPE_ID: AtomicU32 = AtomicU32::new(0);
                let id = TYPE_ID.load(Ordering::Relaxed);
                if id == 0 {
                    let new_id = COMPONENT_TYPE_ID.fetch_add(1, Ordering::Relaxed);
                    TYPE_ID.store(new_id, Ordering::Relaxed);
                    new_id
                } else {
                    id
                }
            }

            fn get_id() -> u32 {
                Self::type_id()
            }
        }
    };
}

pub enum ComponentType {
    Transform(TransformComponent),
    // Health(HealthComponent),
}

struct TransformComponent {
    position: Vector2<f32>,
    scale: Vector2<f32>,
    rotation: f32,
}

impl_component!(TransformComponent);

impl TransformComponent {
    fn new(
        position: Option<Vector2<f32>>,
        scale: Option<Vector2<f32>>,
        rotation: Option<f32>,
    ) -> Self {
        TransformComponent {
            position: position.or(Vector2::new(0.0, 0.0)),
            scale: scale.or(Vector2::new(0.0, 0.0)),
            rotation: rotation.or(0.0),
        }
    }
}

// pub struct HealthComponent {
//     id: u32,
//     data: HealthData,
// }

// struct HealthData {
//     health_percent: i32,
// }

// impl Component for HealthComponent {}

// impl HealthComponent {
//     fn new(data: Option<HealthData>) -> Self {
//         let id = Self::generate_id();
//         let data = match data {
//             Some(d) => d,
//             None => HealthData {
//                 health_percent: 100,
//             },
//         };
//         HealthComponent { id, data }
//     }
//     fn get_id(&self) -> u32 {
//         self.id
//     }
// }
