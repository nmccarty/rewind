//! Provides efficent, immutable storage of a 3D array

use storage::purse::*;
use storage::slice::*;

/// Immutable 3D array with a default value
#[derive(Clone)]
pub struct Cuboid<T> {
    data: Purse<Slice<T>>,
    default: T,
    x_size: usize,
    y_size: usize,
    z_size: usize,
}

impl<T: Clone> Cuboid<T> {
    pub fn new(x_size: usize, y_size: usize, z_size: usize, default: T) -> Cuboid<T> {
        let proto_slice = Slice::new(x_size, y_size, default.clone());
        let purse = Purse::new_filled(z_size, proto_slice);
        Cuboid {
            data: purse,
            default: default,
            x_size: x_size,
            y_size: y_size,
            z_size: z_size,
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> &T {
        if x >= self.x_size || y >= self.y_size || z >= self.z_size {
            &self.default
        } else {
            let slice = &self.data[z];
            slice.get(x, y)
        }
    }

    pub fn set(&self, x: usize, y: usize, z: usize, value: T) -> Option<Cuboid<T>> {
        if x >= self.x_size || y >= self.y_size || z >= self.z_size {
            None
        } else {
            let old_slice = &self.data[z];
            let new_slice = old_slice.set(x, y, value);
            let new_purse = self.data.set(z, new_slice);
            Some(Cuboid {
                data: new_purse,
                default: self.default.clone(),
                x_size: self.x_size,
                y_size: self.y_size,
                z_size: self.z_size,
            })
        }
    }
}
