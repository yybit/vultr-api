### vultr-api

rust implementation of vultr api client

#### Status

- account
  - [x] get_account_info
- billing
  - [x] list_billing_history
  - [x] list_billing_invoices
  - [x] get_billing_invoice
  - [x] list_billing_history
- dns
  - [x] list_dns_domains
  - [x] list_dns_domain_records
- instance
  - [x] list_instances
  - [x] get_instance

#### Usage

Cargo.toml
```
vultr-api = { path = "../vultr-api", features = ["client"] }
```

Example
```rust
let vultr_client = VultrClient::new("https://api.vultr.com/v2".into(), token.into(), Duration::from_secs(10));
vultr_client.list_dns_domains(Some(100), None).await?;
```