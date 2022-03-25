use super::Result;
use serde_json::Value;
use tracing::debug;

pub(crate) fn get_http_client() -> Result<reqwest::Client> {
    let mut client_builder = reqwest::Client::builder();
    if let Ok(proxy) = std::env::var("PROXY") {
        debug!("PROXY is {}", &proxy);
        eprintln!("use proxy: {}", &proxy);
        client_builder = client_builder.proxy(reqwest::Proxy::https(&proxy)?);
    }
    Ok(client_builder.build()?)
}

// signature apifiny token
pub(crate) fn signature_req(conf: &crate::ApiFiny, req: &mut reqwest::Request) -> Result<()> {
    use chrono::{Duration, Utc};
    use reqwest::Method;
    use serde_json::json;

    let mut claims = std::collections::BTreeMap::new();
    claims.insert("accountId", json!(conf.apifiny_account_id));
    claims.insert("secretKeyId", json!(conf.apifiny_access_key));

    // set JWT expiration 24 hours
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp();
    claims.insert("exp", json!(exp));

    // set rest api digest
    use sha2::{Digest, Sha256};
    let digest = match *req.method() {
        Method::GET => req.url().query(),
        Method::POST => {
            if let Some(body) = req.body() {
                let b = body.as_bytes().unwrap_or_default();
                let s = std::str::from_utf8(b)?;
                Some(s)
            } else {
                Some("")
            }
        }
        _ => None,
    };

    if let Some(digest) = digest {
        let mut hasher = Sha256::new();
        hasher.update(digest);
        let digest = format!("{:x}", hasher.finalize());
        claims.insert("digest", json!(digest));
    }

    let token = crate::token::signature(claims, conf.apifiny_secret_key.as_bytes());
    req.headers_mut().append(
        "signature",
        reqwest::header::HeaderValue::from_str(token.as_str())?,
    );

    Ok(())
}

// merge two Value
pub fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}
