#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VultrDomainDTO {
    /// Your registered domain name.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Date the DNS Domain was created.
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The domain's DNSSEC status  * enabled * disabled
    #[serde(rename = "dns_sec", skip_serializing_if = "Option::is_none")]
    pub dns_sec: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VultrDnsRecordDTO {
    /// A unique ID for the DNS Record.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The DNS record type.  * A * AAAA * CNAME * NS * MX * SRV * TXT * CAA * SSHFP
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The hostname for this DNS record.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The DNS data for this record type.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// DNS priority. Does not apply to all record types.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Time to Live in seconds.
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
}
