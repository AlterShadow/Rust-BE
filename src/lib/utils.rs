use convert_case::{Case, Casing};
use eyre::*;
use model::endpoint::EndpointSchema;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_log_id() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as _
}

pub fn get_conn_id() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as _
}

pub fn encode_header<T: Serialize>(v: T, schema: EndpointSchema) -> Result<String> {
    let mut s = String::new();
    write!(s, "0{}", schema.name.to_ascii_lowercase())?;
    let v = serde_json::to_value(&v)?;

    for (i, f) in schema.parameters.iter().enumerate() {
        let key = f.name.to_case(Case::Camel);
        let value = v.get(&key).with_context(|| format!("key: {}", key))?;
        if value.is_null() {
            continue;
        }
        write!(
            s,
            ", {}{}",
            i + 1,
            urlencoding::encode(&value.to_string().replace("\"", ""))
        )?;
    }
    Ok(s)
}

pub fn get_time_seconds() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as _
}

pub fn get_time_milliseconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as _
}
pub fn hex_decode(s: &[u8]) -> Result<Vec<u8>> {
    if s.starts_with(b"0x") {
        Ok(hex::decode(&s[2..])?)
    } else {
        Ok(hex::decode(s)?)
    }
}

#[derive(Serialize)]
struct MailchimpMember {
    email_address: String,
    status: String,
    tags: Vec<String>,
    merge_fields: MailchimpMergeFields,
}

#[derive(Serialize)]
struct MailchimpMergeFields {
    USERNAME: String,
    REFERRER: String,
}

pub async fn subscribe_mailchimp(
    email: &str,
    username: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let member = MailchimpMember {
        email_address: email.to_owned(),
        status: "subscribed".to_owned(),
        tags: vec!["WhitelistRegistration".to_owned()],
        merge_fields: MailchimpMergeFields {
            USERNAME: username.to_owned(),
            REFERRER: "".to_owned(),
        },
    };
    let response = client
        .post("https://us21.api.mailchimp.com/3.0/lists/695c8f4447/members")
        .basic_auth("username", Some("b821122c78b3cb172610c267aa8f164e-us21"))
        .json(&member)
        .send()
        .await?;

    if !response.status().is_success() {
        println!("Request failed with status code: {}", response.status());
        let error_message = response.text().await?;
        println!("Error message: {}", error_message);
        return Ok(());
    } else {
        println!("Mailchimp Success: {}", response.text().await?);
    }
    Ok(())
}
