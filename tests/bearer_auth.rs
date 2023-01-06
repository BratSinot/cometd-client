use cometd_client::types::{access_token::Bearer, AccessToken};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[test]
fn test_basic_auth() {
    let bearer = Bearer::new("Vasya").unwrap();

    assert_eq!(
        bearer.get_authorization_header(),
        HeaderMap::from_iter([(
            HeaderName::from_static("authorization"),
            HeaderValue::from_static("Bearer Vasya")
        )])
    );
}
