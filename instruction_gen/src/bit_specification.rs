/// At the moment, only unsigned types are supported. This enum denotes if something is a
/// single bit or a range of bits and the implicit unsigned type associated.
pub enum BitSpecification
{
    Single(u32),
    Range
    {
        start: u32,
        end: u32,
    },
}
