use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ResolvedComponentItemConfig {
    pub title: String,
    pub reorder_enabled: bool,
    pub component_type: String,
    pub component_state: Value,
    #[serde(flatten)]
    pub headered_item_config: ResolvedHeaderedItemConfig,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ComponentItemConfig<'a> {
    pub title: Option<Cow<'a, str>>,
    pub reorder_enabled: Option<bool>,
    pub component_type: Cow<'a, str>,
    pub component_state: Value,
    #[serde(flatten)]
    pub header_item_config: HeaderItemConfig<'a>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DragSourceComponentItemConfig<'a> {
    #[serde(rename = "type")]
    pub ty: &'a str,
    pub component_state: Value,
    pub component_type: &'a str,
    pub title: Option<&'a str>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedItemConfig {
    #[serde(rename = "type")]
    pub item_type: ItemType,
    pub content: Vec<ResolvedItemConfig>,
    pub size: i32,
    pub size_unit: SizeUnit,
    pub min_size: Option<i32>,
    pub min_size_unit: SizeUnit,
    pub id: String,
    pub is_closable: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemConfig<'a> {
    #[serde(rename = "type")]
    pub item_type: ItemType,
    pub content: Vec<ItemConfig<'a>>,
    pub width: Option<i32>,
    pub min_width: Option<i32>,
    pub height: Option<i32>,
    pub min_height: Option<i32>,
    pub id: Option<Cow<'a, str>>,
    pub is_closable: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizeUnit {
    #[serde(rename = "px")]
    #[default]
    Pixel,
    #[serde(rename = "%")]
    Percent,
    #[serde(rename = "fr")]
    Fractional,
    #[serde(rename = "em")]
    Em,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedHeaderedItemConfig {
    pub header: Option<Header>,
    pub maximized: Option<bool>,
    #[serde(flatten)]
    pub item_config: ResolvedItemConfig,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderItemConfig<'a> {
    pub header: Option<Header>,
    pub maximized: Option<bool>,
    #[serde(flatten)]
    pub item_config: ItemConfig<'a>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Ground,
    Row,
    Column,
    Stack,
    #[default]
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

