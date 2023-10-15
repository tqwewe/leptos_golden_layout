use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentItemConfig {
    pub title: String,
    pub reorder_enabled: bool,
    pub component_type: Value,
    pub component_state: Value,
    #[serde(flatten)]
    pub item_config: ItemConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemConfig {
    #[serde(rename = "type")]
    pub item_type: ItemType,
    pub content: Vec<ItemConfig>,
    pub size: i32,
    pub size_unit: SizeUnit,
    pub min_size: Option<i32>,
    pub min_size_unit: SizeUnit,
    pub id: String,
    pub is_closable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizeUnit {
    #[serde(rename = "px")]
    Pixel,
    #[serde(rename = "%")]
    Percent,
    #[serde(rename = "fr")]
    Fractional,
    #[serde(rename = "em")]
    Em,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderItemConfig {
    pub header: Option<Header>,
    pub maximized: bool,
    #[serde(flatten)]
    pub item_config: ItemConfig,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Ground,
    Row,
    Column,
    Stack,
    Component,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub show: Option<HeaderBoolT<Side>>,
    pub popout: Option<HeaderBoolT<String>>,
    pub maximise: Option<HeaderBoolT<String>>,
    pub close: Option<String>,
    pub minimise: Option<String>,
    pub tab_dropdown: Option<HeaderBoolT<String>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeaderBoolT<T> {
    Bool(bool),
    T(T),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

