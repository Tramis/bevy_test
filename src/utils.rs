use bevy::prelude::*;

pub enum DistanceEnum {
    Zero,
    Lt,
    Eq,
    Gt,
}

pub trait DistanceSquare {
    fn distance_square(&self, other: &Self) -> f32;
}

impl DistanceSquare for Vec2 {
    #[inline]
    fn distance_square(&self, other: &Self) -> f32 {
        (*self - *other).length_squared()
    }
}

pub trait DistanceCmp {
    fn distance_cmp(&self, other: &Self, distance: f32) -> DistanceEnum;
}

impl DistanceCmp for Vec2 {
    #[inline]
    fn distance_cmp(&self, other: &Self, distance: f32) -> DistanceEnum {
        let distance_expected = distance.powi(2);
        let distance = (*self - *other).length_squared();
        
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
