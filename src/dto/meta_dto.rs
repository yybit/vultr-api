#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VultrMetaDTO {
    /// Total objects available in the list. This value may be greater than the number of objects returned if `per_page` is set.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<VultrMetaLinksDTO>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VultrMetaLinksDTO {
    /// Cursor value for the next page.
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// Cursor value for the previous page.
    #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
}
