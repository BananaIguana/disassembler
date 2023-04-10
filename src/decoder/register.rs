use std::fmt::Formatter;

#[allow(non_camel_case_types)]
pub enum Register
{
    x0,
    x1,
    x2,
    x3,
    x4,
    x5,
    x6,
    x7,
    x8,
    x9,
    x10,
    x11,
    x12,
    x13,
    x14,
    x15,
    x16,
    x17,
    x18,
    x19,
    x20,
    x21,
    x22,
    x23,
    x24,
    x25,
    x26,
    x27,
    x28,
    x29,
    x30,
    SP,
}

impl From<u32> for Register
{
    fn from(value: u32) -> Self
    {
        match value
        {
            0 => Register::x0,
            1 => Register::x1,
            2 => Register::x2,
            3 => Register::x3,
            4 => Register::x4,
            5 => Register::x5,
            6 => Register::x6,
            7 => Register::x7,
            8 => Register::x8,
            9 => Register::x9,
            10 => Register::x10,
            11 => Register::x11,
            12 => Register::x12,
            13 => Register::x13,
            14 => Register::x14,
            15 => Register::x15,
            16 => Register::x16,
            17 => Register::x17,
            18 => Register::x18,
            19 => Register::x19,
            20 => Register::x20,
            21 => Register::x21,
            22 => Register::x22,
            23 => Register::x23,
            24 => Register::x24,
            25 => Register::x25,
            26 => Register::x26,
            27 => Register::x27,
            28 => Register::x28,
            29 => Register::x29,
            30 => Register::x30,
            31 => Register::SP,
            _ => panic!("Unhandled"),
        }
    }
}

impl std::fmt::Display for Register
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        let str = match self
        {
            Register::x0 => "x0",
            Register::x1 => "x1",
            Register::x2 => "x2",
            Register::x3 => "x3",
            Register::x4 => "x4",
            Register::x5 => "x5",
            Register::x6 => "x6",
            Register::x7 => "x7",
            Register::x8 => "x8",
            Register::x9 => "x9",
            Register::x10 => "x10",
            Register::x11 => "x11",
            Register::x12 => "x12",
            Register::x13 => "x13",
            Register::x14 => "x14",
            Register::x15 => "x15",
            Register::x16 => "x16",
            Register::x17 => "x17",
            Register::x18 => "x18",
            Register::x19 => "x19",
            Register::x20 => "x20",
            Register::x21 => "x21",
            Register::x22 => "x22",
            Register::x23 => "x23",
            Register::x24 => "x24",
            Register::x25 => "x25",
            Register::x26 => "x26",
            Register::x27 => "x27",
            Register::x28 => "x28",
            Register::x29 => "x29",
            Register::x30 => "x30",
            Register::SP => "sp",
        };

        write!(f, "{}", str)
    }
}
