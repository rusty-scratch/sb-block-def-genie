use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, str::FromStr};

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub struct Blocks(pub Vec<Block>);

impl Blocks {
    pub fn to_string(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }
}

impl FromStr for Blocks {
    type Err = serde_yaml::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(s)
    }
}

#[derive(Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct Block {
    pub identifier: String,
    pub name: String,
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
