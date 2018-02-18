//! Provides an implementation of a single block
//!
//! Uses minecraft stile "provider":"name" format.

use std::collections::HashMap;

/// Structure that stores a single Block
/// Needs to be paired with a BlockDictonary to get useful values
pub struct Block {
    provider: u16,
    blockid: u16,
}

/// Provides a dictonary from provider:blockname values to u16:u16 values
pub struct BlockDictonary {
    provider_to_blocktable: HashMap<String, BlockTable>,
}

/// Provides the table for a single block provider
///
/// In the minecraft blockname "minecraft:air", "minecraft" would be the
/// provider, and "air" would be the name".
pub struct BlockTable {
    provider: String,
    name_to_val: HashMap<String, u16>,
    val_to_name: HashMap<u16, String>,
}

impl BlockTable {
    /// Creates a new, empty blocktable with the given provider name
    pub fn new(provider: String) -> BlockTable {
        BlockTable {
            provider: provider,
            name_to_val: HashMap::new(),
            val_to_name: HashMap::new(),
        }
    }

    /// Adds a name-value pair to the BlockTable
    ///
    /// Warning: will not function properly if you try to rename an existing block
    pub fn add_pair(&mut self, name: &str, val: u16) {
        self.name_to_val.insert(String::from(name), val);
        self.val_to_name.insert(val, String::from(name));
    }

    /// Looks up the value of a block, given the name
    ///
    /// Dangerous, will crash if you give it an invalid name
    pub fn lookup_value(&self, name: &str) -> u16 {
        self.name_to_val.get(name).unwrap().clone()
    }

    /// Looks up the name of a block, given the value
    ///
    /// Dangerous, will crash if given an invalid value
    pub fn lookup_name(&self, val: u16) -> &str {
        self.val_to_name.get(&val).unwrap()
    }
}
