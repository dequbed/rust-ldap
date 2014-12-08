extern crate ldap;

pub fn main()
{
    let ld = *ldap::ldap_init("localhost", 3890u).unwrap();
    let res = ldap::ldap_simple_bind_s(box ld, "cn=admin,dc=nodomain", "DidRPwfLDAP!");
    if res == -1 { panic!(format!("Error in binding to LDAP server: {}", ld.ld_error)); }
    let res2 = ldap::ldap_unbind(box ld);
    println!("{}\n{}", res2, res2);
}
