use hmac::Mac;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

pub fn construct_headers(payload: &str, recv_window: &str) -> HeaderMap {
    let api_key = env::var("testnet_bybit_order_key").expect("BYBIT_API_KEY not set");
    let api_secret = env::var("testnet_bybit_order_secret").expect("BYBIT_API_SECRET not set");
    let current_timestamp = chrono::Utc::now().timestamp_millis().to_string();
    let to_sign = format!(
        "{}{}{}{}",
        &current_timestamp, &api_key, &recv_window, payload
    );

    let signature = {
        type HmacSha256 = hmac::Hmac<sha2::Sha256>;
        let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(to_sign.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        "X-BAPI-API-KEY",
        HeaderValue::from_str(&api_key).expect("Issue processing the api key"),
    );
    headers.insert(
        "X-BAPI-SIGN",
        HeaderValue::from_str(&signature).expect("Issue processing the signature"),
    );
    headers.insert(
        "X-BAPI-TIMESTAMP",
        HeaderValue::from_str(&current_timestamp).expect("Issue processing the timestamp"),
    );
    headers.insert(
        "X-BAPI-RECV-WINDOW",
        HeaderValue::from_str(recv_window).expect("Issue processing the recv window"),
    );
    headers.insert(
        "Connection",
        HeaderValue::from_str("keep-alive").expect("Issue processing the keep alive"),
    );
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("application/json").expect("Issue processing application/json"),
    );
    headers
}

#[cfg(test)]
mod test {
    use super::*;
    use reqwest::header::{HeaderMap, HeaderValue};
    #[test]
    fn test_construct_headers() {
        let symbol = "BTCUSDT";
        let qty = "0.001";

        let current_timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let recv_window = "5000";
        let api_key = env::var("testnet_bybit_order_key").expect("BYBIT_API_KEY not set");
        let api_secret = env::var("testnet_bybit_order_secret").expect("BYBIT_API_SECRET not set");

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-BAPI-API-KEY",
            HeaderValue::from_str(&api_key).expect("Issue processing the api key"),
        );
        headers.insert(
            "X-BAPI-TIMESTAMP",
            HeaderValue::from_str(&current_timestamp).expect("Issue processing the timestamp"),
        );
        headers.insert(
            "X-BAPI-RECV-WINDOW",
            HeaderValue::from_str(recv_window).expect("Issue processing the recv window"),
        );
        headers.insert(
            "Connection",
            HeaderValue::from_str("keep-alive").expect("Issue processing the keep alive"),
        );
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").expect("Issue processing application/json"),
        );

        let payload = format!(
            r#"{{"category":"linear","symbol":"{}","side":"Buy","orderType":"Limit","qty":"{}"}}"#,
            symbol, qty
        );
        let to_sign = format!(
            "{}{}{}{}",
            &current_timestamp, &api_key, &recv_window, payload
        );

        let signature = {
            type HmacSha256 = hmac::Hmac<sha2::Sha256>;
            let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes())
                .expect("HMAC can take key of any size");
            mac.update(to_sign.as_bytes());
            hex::encode(mac.finalize().into_bytes())
        };

        let mut headers_futures = headers.clone();
        headers_futures.insert(
            "X-BAPI-SIGN",
            HeaderValue::from_str(&signature).expect("Issue processing the signature"),
        );

        assert_eq!(construct_headers(&payload, recv_window), headers_futures);

        let payload = "";
        let to_sign = format!(
            "{}{}{}{}",
            &current_timestamp, &api_key, &recv_window, payload
        );

        let signature = {
            type HmacSha256 = hmac::Hmac<sha2::Sha256>;
            let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes())
                .expect("HMAC can take key of any size");
            mac.update(to_sign.as_bytes());
            hex::encode(mac.finalize().into_bytes())
        };

        let mut headers_empty_payload = headers.clone();
        headers_empty_payload.insert(
            "X-BAPI-SIGN",
            HeaderValue::from_str(&signature).expect("Issue processing the signature"),
        );

        assert_eq!(
            construct_headers(payload, recv_window),
            headers_empty_payload
        );
    }
}
