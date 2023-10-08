use nalgebra::Vector2;
use std::any::Any;
use std::sync::atomic::{AtomicU32, Ordering};
static COMPONENT_TYPE_ID: AtomicU32 = AtomicU32::new(1);

pub trait Component: Any {
    fn get_id() -> u32;
}

macro_rules! impl_component {
    ($type:ty) => {
        impl Component for $type {
            fn get_id() -> u32 {
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
        }
    };
}

pub struct TransformComponent {
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
        Self {
            position: position.unwrap_or(Vector2::new(0.0, 0.0)),
            scale: scale.unwrap_or(Vector2::new(0.0, 0.0)),
            rotation: rotation.unwrap_or(0.0),
        }
    }
}

pub struct HealthComponent {
    hp: u16,
    max_hp: u16,
}

impl_component!(HealthComponent);

impl HealthComponent {
    pub fn new(hp: Option<u16>, max_hp: Option<u16>) -> Self {
        Self {
            hp: hp.unwrap_or(100),
            max_hp: max_hp.unwrap_or(100),
        }
    }
}
