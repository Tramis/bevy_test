use bevy::prelude::*;

pub enum DistanceEnum {
    Zero,
    Lt,
    Eq,
    Gt,
}

pub trait DistanceCmp {
    fn distance_cmp(&self, other: Self, distance: f32) -> DistanceEnum;
}

impl DistanceCmp for Vec2 {
    #[inline]
    fn distance_cmp(&self, other: Self, distance: f32) -> DistanceEnum {
        let distance_expected = distance.powi(2);
        let distance = self.distance_squared(other);

        if distance == 0.0 {
            return DistanceEnum::Zero;
        } else if distance == distance_expected {
            return DistanceEnum::Eq;
        } else if distance < distance_expected {
            return DistanceEnum::Lt;
        } else {
            return DistanceEnum::Gt;
        }
    }
}

pub fn random_color() -> Color {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    Color::rgb_u8(
        rng.gen_range(0..180),
        rng.gen_range(0..180),
        rng.gen_range(0..180),
    )
}
