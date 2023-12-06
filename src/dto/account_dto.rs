#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VultrAccountDTO {
    /// Your user name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Your email address.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// An array of permission granted. * manage\\_users * subscriptions_view * subscriptions * billing * support * provisioning * dns * abuse * upgrade * firewall * alerts * objstore * loadbalancer
    #[serde(rename = "acls", skip_serializing_if = "Option::is_none")]
    pub acls: Option<Vec<String>>,
    /// Your current account balance.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<f32>,
    /// Unbilled charges for this month.
    #[serde(rename = "pending_charges", skip_serializing_if = "Option::is_none")]
    pub pending_charges: Option<f32>,
    /// Date of your last payment.
    #[serde(rename = "last_payment_date", skip_serializing_if = "Option::is_none")]
    pub last_payment_date: Option<String>,
    /// The amount of your last payment.
    #[serde(
        rename = "last_payment_amount",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_payment_amount: Option<f32>,
}
