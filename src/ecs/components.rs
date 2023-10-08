use crate::logger::Logger;
use nalgebra::Vector2;
use std::any::Any;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
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
    fn new(hp: Option<u16>, max_hp: Option<u16>) -> Self {
        Self {
            hp: hp.unwrap_or(100),
            max_hp: max_hp.unwrap_or(100),
        }
    }
}

impl<T> Add<T> for HealthComponent
where
    T: Add<Output = T> + Into<u16>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        HealthComponent {
            hp: self.hp.saturating_add(rhs.into()),
            max_hp: self.max_hp,
        }
    }
}

impl<T> AddAssign<T> for HealthComponent
where
    T: Into<u16>,
{
    fn add_assign(&mut self, rhs: T) {
        self.hp = self.hp.saturating_add(rhs.into());
    }
}

impl<T> Sub<T> for HealthComponent
where
    T: Sub<Output = T> + Into<u16>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        HealthComponent {
            hp: self.hp.saturating_sub(rhs.into()),
            max_hp: self.max_hp,
        }
    }
}

impl<T> SubAssign<T> for HealthComponent
where
    T: Into<u16>,
{
    fn sub_assign(&mut self, rhs: T) {
        self.hp = self.hp.saturating_sub(rhs.into());
    }
}

impl<T> Mul<T> for HealthComponent
where
    T: Mul<Output = T> + Into<u16>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        HealthComponent {
            hp: self.hp.saturating_mul(rhs.into()),
            max_hp: self.max_hp,
        }
    }
}

impl<T> MulAssign<T> for HealthComponent
where
    T: Into<u16>,
{
    fn mul_assign(&mut self, rhs: T) {
        self.hp = self.hp.saturating_mul(rhs.into());
    }
}

impl<T> Div<T> for HealthComponent
where
    T: Div<Output = T> + Into<u16>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let divisor = rhs.into();
        if divisor == 0 {
            Logger::warn("Entity hp attempted divide by 0");
            HealthComponent {
                hp: self.max_hp,
                max_hp: self.max_hp,
            }
        } else {
            HealthComponent {
                hp: self.hp.saturating_div(divisor),
                max_hp: self.max_hp,
            }
        }
    }
}

impl<T> DivAssign<T> for HealthComponent
where
    T: Into<u16>,
{
    fn div_assign(&mut self, rhs: T) {
        self.hp = self.hp.saturating_div(rhs.into());
    }
}
