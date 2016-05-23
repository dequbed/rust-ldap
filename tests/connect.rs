extern crate ldap;

use std::thread;
use std::time::Duration;

use ldap::LDAP;
use ldap::Result;


#[test]
fn test_bind() {
    use ldap::bind::{ldap_bind_s, ldap_unbind};

    let mut conn = LDAP::connect("localhost:389").unwrap();

    ldap_bind_s(&mut conn, "cn=root,dc=aicube,dc=renet".to_string(), "secret".to_string());

    thread::sleep(Duration::new(1, 0));

    conn.read();

    ldap_unbind(&mut conn);

    thread::sleep(Duration::new(1, 0));

    conn.read();
}
