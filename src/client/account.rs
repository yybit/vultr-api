use super::*;
use crate::dto::VultrAccountDTO;

impl VultrClient {
    pub async fn get_account(&self) -> Result<VultrAccountDTO> {
        #[derive(Deserialize)]
        struct Response {
            account: Option<VultrAccountDTO>,
        }
        let response: Response = self.get("/account", &[]).await?;
        Ok(response.account.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::VultrClient;

    fn get_client() -> VultrClient {
        let token = std::env::var("VULTR_TOKEN").unwrap();
        VultrClient::new("https://api.vultr.com/v2".into(),
                         token.into(),
                         Duration::from_secs(10))
    }

    #[tokio::test]
    async fn get_account() {
        let client = get_client();
        let result = client.get_account().await;
        assert!(result.is_ok());
    }
}
