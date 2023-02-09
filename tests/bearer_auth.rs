use cometd_client::types::{access_token::Bearer, AccessToken};

#[test]
fn test_basic_auth() {
    let bearer = Bearer::new("Vasya");

    assert_eq!(bearer.get_authorization_token(), "Bearer Vasya");
}
