use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Visibility {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "household")]
    Household,
    #[serde(rename = "org")]
    Org,
    #[serde(rename = "public")]
    Public,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum AccessScope {
    #[serde(rename = "view")]
    View,
    #[serde(rename = "edit")]
    Edit,
    #[serde(rename = "control")]
    Control,
} 