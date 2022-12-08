use cometd_client::{AccessToken, Bearer};

#[test]
fn test_basic_auth() {
    let bearer = Bearer::new("Vasya");

    assert_eq!(
        bearer.get_authorization_header(),
        [("authorization", "Bearer Vasya")]
    );
}
