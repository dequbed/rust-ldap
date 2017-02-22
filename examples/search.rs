extern crate ldap;

use ldap::LdapSync;

pub fn main() {
    let addr = "127.0.0.1:389".parse().unwrap();

    let mut ldap = LdapSync::connect(&addr).unwrap();

    let res = ldap.simple_bind("cn=root,dc=plabs".to_string(), "asdf".to_string()).unwrap();

    if res {
        println!("Bind succeeded!");
        let res2 = ldap.search("dc=plabs".to_string(),
                               ldap::Scope::WholeSubtree,
                               ldap::DerefAliases::Never,
                               false,
                               "(objectClass=*)".to_string());
        println!("Search result: {:?}", res2);
    } else {
        println!("Bind failed! :(");
    }
}
