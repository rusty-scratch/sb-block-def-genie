use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
    categories: Vec<BlockCategory>,
}

#[derive(Deserialize, Serialize)]
pub struct BlockCategory {
    category: String,
    blocks: Vec<Block>,
}

#[derive(Deserialize, Serialize)]
pub struct Block {
    pub name: String,
    pub parameters: Vec<ParameterOrString>,
    pub block_parameters: usize,
}

#[derive(Deserialize, Serialize)]
pub enum ParameterOrString {
    Parameter(Parameter),
    String(String),
}

#[derive(Deserialize, Serialize)]
pub enum Parameter {
    Any,
    Bool,
    EnumOrAny {
        possible_values: BTreeSet<String>,
        restricted_values: BTreeSet<String>,
    },
    Enum {
        possible_values: BTreeSet<String>,
    },
}

#[derive(Deserialize, Serialize)]
pub enum BlockType {
    Hat,
    Stack,
    Cap,
    Reporter,
    Boolean,
    C,
}

fn main() {
    println!("Hello, world!");
}
