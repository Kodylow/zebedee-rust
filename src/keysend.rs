use crate::{StdResp, ZebedeeClient};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysendTx {
    pub id: String,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
    pub r#type: Option<String>,
    #[serde(rename = "totalAmount")]
    pub total_amount: String,
    pub fee: String,
    pub amount: String,
    pub description: Option<String>,
    pub status: String,
    #[serde(rename = "confirmedAt")]
    pub confirmed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysendData {
    #[serde(rename = "keysendId")]
    pub keysend_id: String,
    #[serde(rename = "paymentId")]
    pub payment_id: String,
    pub transaction: KeysendTx,
}

/// Use this struct to create a well crafted json body for your keysend payments

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Keysend {
    pub amount: String,
    pub pubkey: String,
    pub tlv_records: Vec<Option<String>>,
    pub metadata: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
}

pub async fn keysend(
    client: ZebedeeClient,
    keysend_payload: Keysend,
) -> Result<StdResp<Option<KeysendData>>, anyhow::Error> {
    let url = format!("{}/v0/keysend-payment", client.domain);
    let resp = client
        .client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .json(&keysend_payload)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text,
            &url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\n status code: {}",
                e,
                resp_text,
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_keysend() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
        let zebedee_client = ZebedeeClient::new().set_domain(zbdenv).set_apikey(apikey);

        let keysend_payload = Keysend {
            amount: String::from("1000"),
            pubkey: String::from(
                "0332d57355d673e217238ce3e4be8491aa6b2a13f95494133ee243e57df1653ace",
            ),
            ..Default::default()
        };

        let r = keysend(zebedee_client, keysend_payload)
            .await
            .unwrap()
            .success;
        assert!(r);
    }
}
