
pub enum LDAPAuth
{
    NONE,
    SIMPLE,
    SASL,
    KRBv4,
    KRBV41,
    KRBV42,
}

impl LDAPAuth
{
    fn new() -> LDAPAuth { SIMPLE }

}

impl ToPrimitive for LDAPAuth
{
    fn to_uint(&self) -> Option<usize>
    {
        match *self
        {
            NONE => 0x00 as usize,
            SIMPLE => 0x80 as usize,
            SASL => 0xa3 as usize,
            KRBv4 => 0xff as usize,
            KRBV41 => 0x81 as usize,
            KRBV42 => 0x82 as usize,
        }
    }

    fn to_isize(&self) -> Option<isize>
    {
        self.to_uint() as isize
    }

    fn to_i8(&self) -> Option<i8>
    {
        None
    }

    fn to_i16(&self) -> Option<i16>
    {
        self.to_uint() as i16
    }

    fn to_i32(&self) -> Option<i32>
    {
        self.to_uint() as i32
    }

    fn to_i64(&self) -> Option<i64>
    {
        self.to_uint() as i64
    }

    fn to_int(&self) -> Option<isize>
    {
        self.to_uint() as isize
    }

    fn to_usize(&self) -> Option<usize>
    {
        self.to_uint()
    }

    fn to_u8(&self) -> Option<u8>
    {
        self.to_uint() as u8
    }

    fn to_u16(&self) -> Option<u16>
    {
        self.to_uint() as u16
    }
    
    fn to_u32(&self) -> Option<u32>
    {
        self.to_uint() as u32
    }

    fn to_u64(&self) -> Option<u64>
    {
        self.to_uint() as u64
    }

    fn to_f32(&self) -> Option<f32>
    {
        None
    }
    
    fn to_f64(&self) -> Option<f64>
    {
        None
    }
}
