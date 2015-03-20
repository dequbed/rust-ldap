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

    let mut search_result = ldap.search("ou=Benutzer,dc=ad,dc=ggnet", 1, "(objectClass=*)", &["cn", "displayName", "uidNumber"], 0).unwrap();
    let mut first_entry = search_result.first_entry(&mut ldap).unwrap();
    let mut next_entry = search_result.next_entry(&mut ldap).unwrap();
    
    println!("{}", first_entry.get_values(&mut ldap, "cn"));
    println!("{}", next_entry.get_values(&mut ldap, "cn"));
    first_entry.get_values(&mut ldap, "cn");

    ldap.unbind();

}
