extern crate ldap;

use ldap::LdapSync;

pub fn main() {
    let addr = "example.org:636";

    let mut ldap = LdapSync::connect_ssl(&addr).unwrap();

    let res = ldap.simple_bind("cn=root,dc=example,dc=org".to_string(), "secret".to_string()).unwrap();

    if res {
        println!("Bind succeeded!");
    } else {
        println!("Bind failed! :(");
    }
}
