extern crate ldap;

fn main()
{
    let mut ldap = ldap::LDAP::new();

    if !ldap.initialize("ldap://localhost:3890")
    {
        panic!("Init panicked!");
    }

    ldap.set_option();

    if !ldap.simple_bind("cn=admin,dc=ad,dc=ggnet", "DidRPwfLDAP!")
    {
        panic!("Bind panicked!");
    }

    let mut search_result = ldap.search("ou=Benutzer,dc=ad,dc=ggnet", 1, "(objectClass=*)", "*", 0).unwrap();
    println!("{}", search_result.first_entry(&mut ldap).unwrap().get_dn(&mut ldap));
    

    ldap.unbind();

}
