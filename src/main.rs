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

    ldap.destroy();
}
