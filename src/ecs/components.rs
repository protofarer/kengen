// use crate::ecs::Component;
use nalgebra::Vector2;

struct TransformComponent {
    id: u32,
    data: TransformData,
}

struct TransformData {
    position: Vector2<f32>,
    scale: Vector2<f32>,
    rotation: f32,
}

// impl Component for TransformComponent {}

// impl TransformComponent {
//     fn new(data: Option<TransformData>) -> Self {
//         let id = Self::generate_id();
//         let data = match data {
//             Some(d) => d,
//             None => TransformData {
//                 position: Vector2::new(0, 0),
//                 scale: Vector2::new(0, 0),
//                 rotation: 0,
//             },
//         };
//         TransformComponent { id, data }
//     }
//     fn get_id(&self) -> u32 {
//         self.id
//     }
// }

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
