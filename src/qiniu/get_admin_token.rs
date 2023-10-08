use base64::{engine::general_purpose, Engine};
use hmac::{Hmac, Mac};
use sha1::Sha1;

//计算管理资源的token
pub fn get_admin_token(
    method: &str,
    host_name: &str,
    path: &str,
    content_type: &str,
    access_key: &str,
    secret_key: &str,
) -> String {
    let mut signing_str = format!("{} {}\nHost: {}", method, path, host_name);
    if !content_type.is_empty() {
        signing_str = format!("{}\nContent-Type: {}", signing_str, &content_type);
    }
    signing_str = format!("{}\n\n", signing_str);
    let mut mac =
        Hmac::<Sha1>::new_from_slice(secret_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(signing_str.as_bytes());
    let sign_result = mac.finalize();
    let sign_result = sign_result.into_bytes();
    let encoded_sign = general_purpose::URL_SAFE.encode(sign_result);
    format!("{access_key}:{encoded_sign}")
}
