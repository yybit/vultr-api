#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VultrBillingDTO {
    /// ID of the billing history item
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    /// Date billing history item was generated
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Type of billing history item
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Description of billing history item
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Amount for the billing history item in dollars
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// The accounts balance in dollars
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BillingInvoiceItemModel {
    /// Invoice item description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Product name
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// Start date of item
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// End date of item
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Number of units item consumed in billing period
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<f32>,
    /// Unit type. Options include \"hours\", \"overage\", and \"discount\"
    #[serde(rename = "unit_type", skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
    /// Price per unit in dollars
    #[serde(rename = "unit_price", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f32>,
    /// Total amount due in dollars
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f32>,
}
