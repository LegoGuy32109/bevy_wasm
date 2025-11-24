use bevy::prelude::*;

const WORLD_SIZE: UVec2 = UVec2::splat(16);

#[derive(Component, Debug)]
pub struct MapCoordinates {
    origin: IVec3,
}

impl MapCoordinates {
    pub fn add_ivec3(&mut self, vec: IVec3) -> &mut Self {
        self.origin += vec;
        self
    }

    /// given tilemap indexed coordinates, convert to map / world coordinates
    pub fn from_uvec2(index: UVec2) -> Self {
        Self {
            origin: IVec3 {
                x: i32::try_from(index.x).unwrap() - i32::try_from(WORLD_SIZE.x).unwrap() / 2,
                y: i32::try_from(index.y).unwrap() - i32::try_from(WORLD_SIZE.y).unwrap() / 2,
                z: 0,
            },
        }
    }

    /// convert internal map / world coordinates to tilemap indexed coordinates
    pub fn as_uvec2(&self) -> UVec2 {
        UVec2 {
            x: u32::try_from(self.origin.x + i32::try_from(WORLD_SIZE.x).unwrap() / 2).unwrap(),
            y: u32::try_from(self.origin.y + i32::try_from(WORLD_SIZE.y).unwrap() / 2).unwrap(),
        }
    }
}

impl Clone for MapCoordinates {
    fn clone(&self) -> Self {
        Self {
            origin: self.origin.clone(),
        }
    }
}
