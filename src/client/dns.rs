use super::*;
use crate::dto::{VultrDnsRecordDTO, VultrDomainDTO};

impl VultrClient {
    pub async fn list_dns_domains(&self, per_page: Option<i32>,
                              cursor: Option<&str>) -> Result<Vec<VultrDomainDTO>> {
        #[derive(Deserialize)]
        struct Response {
            domains: Option<Vec<VultrDomainDTO>>,
            // meta: Option<VultrMetaDTO>,
        }
        let per_page_str = per_page.map(|i| i.to_string());
        let params = [
            ("per_page", per_page_str.as_deref()),
            ("cursor", cursor),
        ];
        let response: Response = self.get("/domains", &params).await?;
        Ok(response.domains.unwrap_or_default())
    }

    pub async fn list_dns_domain_records(&self, dns_domain: &str, per_page: Option<i32>,
                                     cursor: Option<&str>) -> Result<Vec<VultrDnsRecordDTO>> {
        #[derive(Deserialize)]
        struct Response {
            records: Option<Vec<VultrDnsRecordDTO>>,
            // meta: Option<VultrMetaDTO>,
        }
        let per_page_str = per_page.map(|i| i.to_string());
        let params = [
            ("per_page", per_page_str.as_deref()),
            ("cursor", cursor),
        ];
        let response: Response = self.get(format!("/domains/{}/records", dns_domain).as_str(), &params).await?;
        Ok(response.records.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::VultrClient;

    fn get_client() -> VultrClient {
        let token = std::env::var("VULTR_TOKEN").unwrap();
        VultrClient::new("https://api.vultr.com/v2".into(),
                         token.into(),
                         Duration::from_secs(10))
    }

    #[tokio::test]
    async fn list_dns_domains() {
        let client = get_client();
        let result = client.list_dns_domains(Some(10), None)
            .await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn list_dns_domain_records() {
        let client = get_client();
        let result = client.list_dns_domain_records("rusttool.com",
                                                    Some(10), None).await;
        assert!(result.is_ok())
    }
}
