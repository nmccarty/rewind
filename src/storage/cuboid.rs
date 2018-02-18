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
