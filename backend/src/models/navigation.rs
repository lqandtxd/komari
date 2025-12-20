use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

use super::impl_identifiable;

#[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
pub struct NavigationPaths {
    #[serde(skip_serializing, default)]
    pub id: Option<i64>,
    pub name: String,
    pub paths: Vec<NavigationPath>,
}

impl_identifiable!(NavigationPaths);

#[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
pub struct NavigationPath {
    pub minimap_snapshot_base64: String,
    #[serde(default)]
    pub minimap_snapshot_grayscale: bool,
    pub name_snapshot_base64: String,
    pub name_snapshot_width: i32,
    pub name_snapshot_height: i32,
    pub points: Vec<NavigationPoint>,
}

#[derive(PartialEq, Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct NavigationPoint {
    // Not FK, loose coupling to another navigation paths and its index
    pub next_paths_id_index: Option<(i64, usize)>,
    pub x: i32,
    pub y: i32,
    pub transition: NavigationTransition,
}

#[derive(
    Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize, EnumIter, Display, EnumString,
)]
pub enum NavigationTransition {
    #[default]
    Portal,
}
