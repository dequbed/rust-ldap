extern crate ldap_client as ldap;

use std::thread;
use std::time::Duration;

use ldap::LDAP;
use ldap::Result;


#[test]
fn test_bind() {
    use ldap::bind::{ldap_bind, ldap_unbind};

    let mut conn = LDAP::connect("localhost:389").unwrap();

    ldap_bind(&mut conn, "cn=root,dc=aicube,dc=renet".to_string(), "secret".to_string());

    thread::sleep_ms(2000);

    let tag = conn.recv().unwrap();

    ldap_unbind(&mut conn);
}
