use cometd_client::types::{access_token::Basic, AccessToken};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[test]
fn test_basic_auth() {
    let basic0 = Basic::create("Vasya", None).unwrap();
    let basic1 = Basic::create("Vasya", Some("Petya")).unwrap();

    assert_eq!(
        basic0.get_authorization_header(),
        HeaderMap::from_iter([(
            HeaderName::from_static("authorization"),
            HeaderValue::from_static("Basic VmFzeWE6")
        )])
    );
    assert_eq!(
        basic1.get_authorization_header(),
        HeaderMap::from_iter([(
            HeaderName::from_static("authorization"),
            HeaderValue::from_static("Basic VmFzeWE6UGV0eWE=")
        )])
    );
}
