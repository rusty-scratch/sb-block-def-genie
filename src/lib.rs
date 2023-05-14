use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, str::FromStr};

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub struct Blocks(pub Vec<Block>);

#[derive(Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Block {
    pub identifier: String,
    pub opcode: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub doc: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementor_note: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub manual: bool,
    pub block_type: BlockType,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterOrText>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub enum ParameterOrText {
    Parameter(Parameter),
    Text(String),
}

impl From<Parameter> for ParameterOrText {
    fn from(value: Parameter) -> Self {
        ParameterOrText::Parameter(value)
    }
}

impl From<String> for ParameterOrText {
    fn from(value: String) -> Self {
        ParameterOrText::Text(value)
    }
}

#[derive(Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Parameter {
    /// name will be use in rust and also unique in the repository
    pub identifier: String,
    pub key: String,
    pub parameter_type: ParameterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deny_strings: Option<BTreeSet<String>>,
}

#[derive(Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum ParameterType {
    Any,
    Number,
    PositiveInteger,
    Integer,
    Angle,
    Color,
    #[default]
    String,
    UnrestrictedField {
        menu_block: String,
    },
    Boolean,
    Field {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        possible_values: Option<BTreeSet<String>>,
        /// Because unknown value
        #[serde(default, skip_serializing_if = "Option::is_none")]
        possible_categories: Option<BTreeSet<String>>,
    },
    Block,
}

#[derive(Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum BlockType {
    Hat,
    #[default]
    Stack,
    Cap,
    Reporter {
        return_type: ParameterType,
    },
    C,
    Menu,
}

pub fn from_str(str: &str) -> serde_yaml::Result<Blocks> {
    serde_yaml::from_str(str)
}

pub fn from_reader<R>(reader: R) -> serde_yaml::Result<Blocks>
where
    R: std::io::Read,
{
    serde_yaml::from_reader(reader)
}

pub fn from_slice(slice: &[u8]) -> serde_yaml::Result<Blocks> {
    serde_yaml::from_slice(slice)
}

pub fn from_yaml_value(yaml: serde_yaml::Value) -> serde_yaml::Result<Blocks> {
    serde_yaml::from_value(yaml)
}

pub fn to_string(blocks: &Blocks) -> serde_yaml::Result<String> {
    serde_yaml::to_string(blocks)
}

pub fn to_yaml_value(blocks: &Blocks) -> serde_yaml::Result<serde_yaml::Value> {
    serde_yaml::to_value(blocks)
}

pub fn to_writer<W>(writer: W, blocks: &Blocks) -> serde_yaml::Result<()>
where
    W: std::io::Write,
{
    serde_yaml::to_writer(writer, blocks)
}
