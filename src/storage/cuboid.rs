//! Provides efficent, immutable storage of a 3D array

use storage::purse::*;
use storage::slice::*;

/// Immutable 3D array with a default value
pub struct Cuboid<T> {
    data: Purse<Slice<T>>,
    default: T,
    x_size: usize,
    y_size: usize,
    z_size: usize,
}

impl<T: Clone> Cuboid<T> {
    fn new(x_size: usize, y_size: usize, z_size: usize, default: T) -> Cuboid<T> {
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
}
