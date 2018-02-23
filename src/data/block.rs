//! Provides an implementation of a single block
//!
//! Uses minecraft stile "provider":"name" format.

use std::collections::HashMap;

/// Structure that stores a single Block
/// Needs to be paired with a BlockDictonary to get useful values
#[derive(Clone, Copy)]
pub struct Block {
    provider: u16,
    id: u16,
}

impl Block {
    pub fn new_from_ids(provider: u16, id: u16) -> Block {
        Block {
            provider: provider,
            id: id,
        }
    }
}

/// Stores metadata about a block (i.e. damagevalue)
#[derive(Clone, Copy)]
pub struct MetaData {
    data_value: Option<i32>,
}

impl MetaData {
    /// Creates a new MetaData with nothing in it
    pub fn new() -> MetaData {
        MetaData { data_value: None }
    }

    /// Sets the data_value of the meta data
    pub fn set_data_value(&self, data_value: i32) -> MetaData {
        let mut new_meta = self.clone();
        new_meta.data_value = Some(data_value);
        new_meta
    }

    /// Gets the data_value of the meta data
    pub fn get_data_value(&self) -> Option<i32> {
        self.data_value
    }
}

/// Pairs a block with its metadata, if it has any
#[derive(Clone, Copy)]
pub struct MetaBlock {
    block: Block,
    meta_data: MetaData,
}

impl MetaBlock {
    /// Combines a block and a metadata into a metablock
    pub fn fuse(block: Block, meta: MetaData) -> MetaBlock {
        MetaBlock {
            block: block,
            meta_data: meta,
        }
    }

    /// Returns the block of the pair
    pub fn get_block(&self) -> &Block {
        &self.block
    }

    /// Returns the meta data of the pair
    pub fn get_meta_data(&self) -> &MetaData {
        &self.meta_data
    }
}

/// Provides a dictonary from provider:blockname values to u16:u16 values
pub struct BlockDictonary {
    provider_id_to_blocktable: HashMap<u16, BlockTable>,
    provider_name_to_id: HashMap<String, u16>,
    provider_id_to_name: HashMap<u16, String>,
}

impl BlockDictonary {
    /// Creates a new, empty, BlockDictonary
    pub fn new() -> BlockDictonary {
        BlockDictonary {
            provider_id_to_blocktable: HashMap::new(),
            provider_id_to_name: HashMap::new(),
            provider_name_to_id: HashMap::new(),
        }
    }

    /// Returns an avaible provider id
    fn new_id(&self) -> u16 {
        let new_val = self.provider_id_to_name.keys().max();
        if let Some(&x) = new_val {
            x + 1
        } else {
            0
        }
    }

    /// Adds a blocktable to the dictionary with the specificed id
    ///
    /// Dangerous, won't behave properly if you rename an existing blocktable with it
    pub fn add_pair(&mut self, table: BlockTable, id: u16) {
        self.provider_id_to_name.insert(id, table.provider.clone());
        self.provider_name_to_id.insert(table.provider.clone(), id);
        self.provider_id_to_blocktable.insert(id, table);
    }

    /// Adds a blocktable to the dictionary
    pub fn add_table(&mut self, table: BlockTable) -> u16 {
        let id = self.new_id();
        self.add_pair(table, id);
        id
    }

    /// Provides a ("provider","id") from a block
    ///
    /// Dangerous, will crash if given a malformed block
    pub fn decode_block(&self, block: Block) -> (&str, &str) {
        let table = self.provider_id_to_blocktable.get(&block.provider).unwrap();
        let id = table.lookup_name(block.id);
        (table.get_provider(), id)
    }

    /// Provides a block from a ("provider","id")
    ///
    /// Dangerous, will crash if given a bad ("Provider","id")
    pub fn encode_block(&self, (provider, id): (&str, &str)) -> Block {
        let provider_id = self.provider_name_to_id.get(provider).unwrap();
        let blocktable = self.provider_id_to_blocktable.get(provider_id).unwrap();
        let block_id = blocktable.lookup_value(id);
        Block::new_from_ids(*provider_id, block_id)
    }
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
    pub fn new(provider: &str) -> BlockTable {
        BlockTable {
            provider: String::from(provider),
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

    /// Returns the next available id
    fn new_id(&self) -> u16 {
        let new_val = self.val_to_name.keys().max();
        if let Some(&x) = new_val {
            x + 1
        } else {
            0
        }
    }

    /// Adds a name to the blocktable, assinging the next avaible value
    pub fn add_name(&mut self, name: &str) -> u16 {
        let id = self.new_id();
        self.add_pair(name, id);
        id
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

    /// Returns the name of this blocktable
    pub fn get_provider(&self) -> &str {
        &self.provider
    }
}
