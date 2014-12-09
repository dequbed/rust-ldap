extern crate ldap;

pub fn main()
{
    let ld = *ldap::ldap_init("localhost", 3890u).unwrap();
    let res3 = ldap::ldap_set_option(box ld, 0x0011, 3);
    if res3 != 0 { panic!(format!("Error in setting LDAP version: {} {}", ld.ld_error, res3)); }
    let res = ldap::ldap_simple_bind_s(box ld, "cn=admin,dc=nodomain", "DidRPwfLDAP!");
    if res != 0 { panic!(format!("Error in binding to LDAP server: {} {}", ld.ld_error, res)); }
    let res2 = ldap::ldap_unbind(box ld);
    println!("{}\n{}", res2, res2);
}
