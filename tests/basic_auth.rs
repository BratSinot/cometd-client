use cometd_client::{AccessToken, Basic};

#[test]
fn test_basic_auth() {
    let basic0 = Basic::create("Vasya", None).unwrap();
    let basic1 = Basic::create("Vasya", Some("Petya")).unwrap();

    assert_eq!(
        basic0.get_authorization_header(),
        [("authorization", "Basic VmFzeWE6")]
    );
    assert_eq!(
        basic1.get_authorization_header(),
        [("authorization", "Basic VmFzeWE6UGV0eWE=")]
    );
}
