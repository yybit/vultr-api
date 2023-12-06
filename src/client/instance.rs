use super::*;
use crate::dto::VultrInstanceDTO;

impl VultrClient {
    pub async fn list_instances(&self, per_page: Option<i32>, cursor: Option<&str>,
                            tag: Option<&str>, label: Option<&str>,
                            main_ip: Option<&str>, region: Option<&str>) -> Result<Vec<VultrInstanceDTO>> {
        #[derive(Deserialize)]
        struct Response {
            instances: Option<Vec<VultrInstanceDTO>>,
            // meta: Option<VultrMetaDTO>,
        }
        let per_page_str = per_page.map(|i| i.to_string());
        let params = [
            ("per_page", per_page_str.as_deref()),
            ("cursor", cursor),
            ("tag", tag),
            ("label", label),
            ("main_ip", main_ip),
            ("region", region),
        ];
        let response: Response = self.get("/instances", &params).await?;
        Ok(response.instances.unwrap_or_default())
    }

    pub async fn get_instance(&self, instance_id: &str) -> Result<VultrInstanceDTO> {
        #[derive(Deserialize)]
        struct Response {
            instance: Option<VultrInstanceDTO>,
        }
        let response: Response = self.get(format!("/instances/{}", instance_id).as_str(), &[]).await?;
        Ok(response.instance.unwrap_or_default())
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
    async fn list_instances() {
        let client = get_client();
        let result = client.list_instances(Some(10),
                                           None, None, None,
                                           None, None).await;
        assert!(result.is_ok())
    }
}