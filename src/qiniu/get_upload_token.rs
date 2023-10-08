use super::PutPolicy;
use base64::engine::general_purpose;
use base64::Engine;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::ops::Add;
use std::time::{Duration, SystemTime};

pub fn get_upload_token(
    bucket_name: &str,
    save_path: &str,
    access_key: &str,
    secret_key: &str,
) -> String {
    let scope = format!("{}:{}", bucket_name, save_path);
    let deadline = {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let deadline = now.add(Duration::from_secs(300));
        deadline.as_secs()
    };
    let return_body = "{
    \"name\": $(fname),
    \"size\": $(fsize),
    \"w\": $(imageInfo.width),
    \"h\": $(imageInfo.height),
    \"hash\": $(etag)
}";
    let put_policy = PutPolicy {
        scope,
        deadline,
        return_body: return_body.to_string(),
    };
    let put_policy = serde_json::to_string(&put_policy).unwrap();
    let encoded_put_policy = general_purpose::URL_SAFE.encode(put_policy.as_bytes());
    let mut mac =
        Hmac::<Sha1>::new_from_slice(secret_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(encoded_put_policy.as_bytes());
    let sign_result = mac.finalize();
    let sign_result = sign_result.into_bytes();
    let encoded_sign = general_purpose::URL_SAFE.encode(sign_result);
    format!("{}:{}:{}", access_key, &encoded_sign, &encoded_put_policy)
}
