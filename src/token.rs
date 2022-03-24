use hmac::{Hmac, Mac};
use jwt::{token::Signed, AlgorithmType, Header, SignWithKey, Token};
use serde_json::Value;
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn signature<'a>(
    claims: BTreeMap<&'a str, Value>,
    secret: &'a [u8],
) -> Token<Header, BTreeMap<&'a str, Value>, Signed> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret).unwrap();
    let header = Header {
        algorithm: AlgorithmType::Hs256,
        ..Default::default()
    };

    Token::new(header, claims).sign_with_key(&key).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token() {
        use serde_json::json;

        let mut claims = BTreeMap::new();
        claims.insert("sub", json!("someone"));
        let token = signature(claims, b"some-secret");
        assert_eq!(token.as_str(), "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0");
    }
}
