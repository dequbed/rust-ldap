extern crate byteorder;

pub mod ber;
mod bind;
// mod connection;

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_bind()
    {
        bind::ldap_bind_s
    }
}
