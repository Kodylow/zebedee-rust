use crate::{StdResp, ZebedeeClient};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsData {
    pub id: String,
    pub fee: Option<String>,
    pub unit: String,
    pub amount: String,
    pub invoice: Option<String>,
    pub preimage: Option<String>,
    #[serde(rename = "internalId")]
    pub internal_id: Option<String>,
    #[serde(rename = "processedAt")]
    pub processed_at: Option<DateTime<Utc>>,
    #[serde(rename = "confirmedAt")]
    pub confirmed_at: Option<DateTime<Utc>>,
    pub description: String,
    pub status: String,
}

/// Use this struct to create a well crafted json body for normal ligthning bolt 11 payments
#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub description: String,
    #[serde(rename = "internalId")]
    pub internal_id: String,
    pub invoice: String,
}

impl Default for Payment {
    fn default() -> Payment {
        Payment {
            description: String::from("using zebedee rust sdk"),
            internal_id: String::from(""),
            invoice: String::from(""),
        }
    }
}

pub async fn pay_invoice(
    client: ZebedeeClient,
    payment: Payment,
) -> Result<StdResp<Option<PaymentsData>>, anyhow::Error> {
    let resp = client
        .client
        .post(format!("{}/v0/payments", client.domain))
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .json(&payment)
        .send()
        .await?;

    let status_code = resp.status();

    let resp_text = resp.text().await?;

    match status_code {
        reqwest::StatusCode::OK => (), //dbg!("OK status:"),
        s => {
            return Err(anyhow::anyhow!(
                "Error: status {}, message: {}",
                s,
                resp_text
            ));
        }
    };

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

pub async fn get_payments(
    client: ZebedeeClient,
) -> Result<StdResp<Option<Vec<PaymentsData>>>, anyhow::Error> {
    let resp = client
        .client
        .get(format!("{}/v0/payments", client.domain))
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();
    let resp_text = resp.text().await?;

    match status_code {
        reqwest::StatusCode::OK => (), //dbg!("OK status:"),
        s => {
            return Err(anyhow::anyhow!(
                "Error: status {}, message: {}",
                s,
                resp_text
            ));
        }
    };

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

pub async fn get_payment(
    client: ZebedeeClient,
    payment_id: String,
) -> Result<StdResp<Option<PaymentsData>>, anyhow::Error> {
    let url = format!("{}/v0/payments/{}", client.domain, payment_id);
    let resp = client
        .client
        .get(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .send()
        .await?;

    let status_code = resp.status();

    let resp_text = resp.text().await?;

    match status_code {
        reqwest::StatusCode::OK => (), //dbg!("OK status:"),
        s => {
            return Err(anyhow::anyhow!(
                "Error: status {}, message: {}, payment_id: {}, url: {}",
                s,
                resp_text,
                payment_id,
                &url,
            ));
        }
    };

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\nstatus code: {}\npayment_requests_id: {}\n url: {}",
                e,
                resp_text,
                status_code,
                payment_id,
                &url,
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
    async fn test_pay_invoice() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
        let zebedee_client = ZebedeeClient::new().set_domain(zbdenv).set_apikey(apikey);

        let payment = Payment {
            invoice: String::from("lnbc120n1p0tdjwmpp5ycws0d788cjeqp9rn2wwxfymrekj9n80wy2yrk66tuu3ga5wukfsdzq2pshjmt9de6zqen0wgsrzv3qwp5hsetvwvsxzapqwdshgmmndp5hxtnsd3skxefwxqzjccqp2sp5vnsvmjlu6hrfegcdjs47njrga36g3x45wfmqjjjlerwgagj62yysrzjq2v4aw4gy7m93en32dcaplym056zezcljdjshyk8yakwtsp2h4yvcz9atuqqhtsqqqqqqqlgqqqqqqgqjq9qy9qsqhykfacrdy06cuyegvt4p50su53qwgrqn5jf6d83fd0upsa4frpxqnm2zl323zuvmz5ypv9gh9nr3jav6u2ccwkpd56h3n6l3ja5q7wgpxudlv4"),
            ..Default::default()
        };
        // expected to get a 400 error
        let r = pay_invoice(zebedee_client, payment).await.err().unwrap();
        assert!(r.to_string().contains("400"));
    }
    #[tokio::test]
    async fn test_get_payments() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
        let zebedee_client = ZebedeeClient::new().set_domain(zbdenv).set_apikey(apikey);

        let r = get_payments(zebedee_client).await.unwrap();
        assert!(r.success);
    }
    #[tokio::test]
    async fn test_get_payment() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
        let zebedee_client = ZebedeeClient::new().set_domain(zbdenv).set_apikey(apikey);

        let payment_id = String::from("5d88b2e0-e491-40e1-a8a8-a81ae68f2297");

        let r = get_payment(zebedee_client, payment_id).await.err().unwrap();
        assert!(r.to_string().contains("404"));
    }
}
