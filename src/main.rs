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

    //let mut search_result = ldap.search("ou=Benutzer,dc=ad,dc=ggnet", 1, "(&(cn=testuser)(objectClass=*))", &["cn", "displayName", "uidNumber"], 0).unwrap();
    let mut search_result = ldap.search("ou=Benutzer,dc=ad,dc=ggnet", 1, "(&(cn=*)(objectClass=person))", &["cn", "memberOf"], 0).unwrap();
    let mut first_entry = search_result.first_entry(&mut ldap).unwrap();
    //let mut next_entry = search_result.next_entry(&mut ldap).unwrap();
    //let mut nexts_entry = next_entry.next_entry(&mut ldap).unwrap();
    
    println!("{}", search_result.count_entries(&mut ldap));
    println!("{:?}", first_entry.get_values(&mut ldap, "memberOf"));
    //println!("{}", next_entry.get_values(&mut ldap, "cn"));
    //println!("{}", nexts_entry.get_values(&mut ldap, "cn"));

    ldap.unbind();

}
