extern crate ldap;

pub fn main()
{
    let ld = ldap::ldap_init("localhost", 3890u).unwrap();
    let res = ldap::ldap_simple_bind(ld, "cn=admin,dc=nodomain", "DidRPwfLDAP!");
    let res2 = ldap::ldap_unbind(ld);
    println!("{}\n{}", res2, res2);
}
