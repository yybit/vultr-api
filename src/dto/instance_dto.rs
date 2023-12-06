#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VultrInstanceDTO {
    /// A unique ID for the VPS Instance.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The [Operating System name](#operation/list-os).
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// The amount of RAM in MB.
    #[serde(rename = "ram", skip_serializing_if = "Option::is_none")]
    pub ram: Option<i32>,
    /// The size of the disk in GB.
    #[serde(rename = "disk", skip_serializing_if = "Option::is_none")]
    pub disk: Option<i32>,
    /// The main IPv4 address.
    #[serde(rename = "main_ip", skip_serializing_if = "Option::is_none")]
    pub main_ip: Option<String>,
    /// Number of vCPUs.
    #[serde(rename = "vcpu_count", skip_serializing_if = "Option::is_none")]
    pub vcpu_count: Option<i32>,
    /// The [Region id](#operation/list-regions) where the Instance is located.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The default password assigned at deployment.
    #[serde(rename = "default_password", skip_serializing_if = "Option::is_none")]
    pub default_password: Option<String>,
    /// The date this instance was created.
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The current status.  * active * pending * suspended * resizing
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The power-on status.  * running * stopped
    #[serde(rename = "power_status", skip_serializing_if = "Option::is_none")]
    pub power_status: Option<String>,
    /// The server health status.  * none * locked * installingbooting * ok
    #[serde(rename = "server_status", skip_serializing_if = "Option::is_none")]
    pub server_status: Option<String>,
    /// Monthly bandwidth quota in GB.
    #[serde(rename = "allowed_bandwidth", skip_serializing_if = "Option::is_none")]
    pub allowed_bandwidth: Option<i32>,
    /// The IPv4 netmask in dot-decimal notation.
    #[serde(rename = "netmask_v4", skip_serializing_if = "Option::is_none")]
    pub netmask_v4: Option<String>,
    /// The gateway IP address.
    #[serde(rename = "gateway_v4", skip_serializing_if = "Option::is_none")]
    pub gateway_v4: Option<String>,
    /// An array of IPv6 objects.
    #[serde(rename = "v6_networks", skip_serializing_if = "Option::is_none")]
    pub v6_networks: Option<Vec<VultrInstanceV6NetworksDTO>>,
    /// The hostname for this instance.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The user-supplied label for this instance.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The user-supplied tag for this instance.
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// The [user data](https://www.vultr.com/docs/manage-instance-user-data-with-the-vultr-metadata-api) that can be supplied for tools such as cloudinit.
    #[serde(rename = "internal_ip", skip_serializing_if = "Option::is_none")]
    pub internal_ip: Option<String>,
    /// HTTPS link to the Vultr noVNC Web Console.
    #[serde(rename = "kvm", skip_serializing_if = "Option::is_none")]
    pub kvm: Option<String>,
    /// The [Operating System id](#operation/list-os) used by this instance.
    #[serde(rename = "os_id", skip_serializing_if = "Option::is_none")]
    pub os_id: Option<i32>,
    /// The [Application id](#operation/list-applications) used by this instance.
    #[serde(rename = "app_id", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i32>,
    /// The [Application image_id](#operation/list-applications) used by this instance.
    #[serde(rename = "image_id", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// The [Firewall Group id](#operation/list-firewall-groups) linked to this Instance.
    #[serde(rename = "firewall_group_id", skip_serializing_if = "Option::is_none")]
    pub firewall_group_id: Option<String>,
    /// \"auto_backups\", \"ipv6\", \"ddos_protection\"
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    /// A unique ID for the Plan.
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VultrInstanceV6NetworksDTO {
    /// The IPv6 subnet.
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// The main IPv6 network address.
    #[serde(rename = "main_ip", skip_serializing_if = "Option::is_none")]
    pub main_ip: Option<String>,
    /// The IPv6 network size in bits.
    #[serde(rename = "network_size", skip_serializing_if = "Option::is_none")]
    pub network_size: Option<i32>,
}
