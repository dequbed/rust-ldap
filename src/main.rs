extern crate ldap;

pub fn main()
{
    let ld = ldap::ldap_init("localhost", 3891u).unwrap();
    let res = ldap::ldap_bind(&ld, "cn=admin, dc=example, dc=com", "DidRPwfLDAP!", 0x80);
    println!("{}", res);
}
