use super::*;
use crate::dto::{VultrBillingDTO, BillingInvoiceItemModel};

impl VultrClient {
    pub async fn list_billing_history(&self) -> Result<Vec<VultrBillingDTO>> {
        #[derive(Deserialize)]
        struct Response {
            billing_history: Option<Vec<VultrBillingDTO>>,
        }
        let response: Response = self.get("/billing/history", &[]).await?;
        Ok(response.billing_history.unwrap_or_default())
    }

    pub async fn list_billing_invoices(&self) -> Result<Vec<VultrBillingDTO>> {
        #[derive(Deserialize)]
        struct Response {
            billing_invoices: Option<Vec<VultrBillingDTO>>,
        }
        let response: Response = self.get("/billing/invoices", &[]).await?;
        Ok(response.billing_invoices.unwrap_or_default())
    }

    pub async fn get_billing_invoice(&self, invoice_id: &str) -> Result<VultrBillingDTO> {
        self.get(format!("/billing/invoices/{}", invoice_id).as_str(), &[]).await
    }

    pub async fn get_billing_invoice_items(&self, invoice_id: &str) -> Result<BillingInvoiceItemModel> {
        self.get(format!("/billing/invoices/{}/items", invoice_id).as_str(), &[]).await
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
    async fn list_billing_history() {
        let client = get_client();
        let result = client.list_billing_history().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn list_billing_invoices() {
        let client = get_client();
        let result = client.list_billing_invoices().await;
        assert!(result.is_ok());
    }
}
