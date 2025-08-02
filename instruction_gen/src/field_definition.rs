use {
    crate::bit_specification::BitSpecification,
    syn::{
        Ident,
        LitInt,
        LitStr,
        Token,
        parse::{Parse, ParseStream, Result},
    },
};

/// Represents parsing of `sf = 1` or `imm12 = 10:21`
pub struct FieldDefinition
{
    pub name: Ident,
    _eq: Token![=],
    pub bit_spec: BitSpecification,
}

impl Parse for FieldDefinition
{
    fn parse(input: ParseStream) -> Result<Self>
    {
        let name = input.parse()?;
        let eq = input.parse()?;

        if input.peek(LitStr)
        {
            let lit: LitStr = input.parse()?;
            let pattern = lit.value();
            if pattern.len() != 32
            {
                return Err(input.error("mask pattern must be 32 characters"));
            }
            let mut mask_val = 0u32;
            let mut check_val = 0u32;
            for (i, c) in pattern.chars().enumerate()
            {
                if c != '*'
                {
                    mask_val |= 1 << (31 - i);
                    if c == '1'
                    {
                        check_val |= 1 << (31 - i);
                    }
                }
            }
            Ok(FieldDefinition {
                name,
                _eq: eq,
                bit_spec: BitSpecification::Mask {
                    pattern,
                    mask: mask_val,
                    check: check_val,
                },
            })
        }
        else
        {
            let first: LitInt = input.parse()?;
            let bit_spec = if input.peek(Token![:])
            {
                let _: Token![:] = input.parse()?;
                let second: LitInt = input.parse()?;
                let start = first.base10_parse()?;
                let end = second.base10_parse()?;
                if end < start
                {
                    return Err(input.error("range end must be ≥ start"));
                }
                BitSpecification::Range { start, end }
            }
            else
            {
                BitSpecification::Single(first.base10_parse()?)
            };
            Ok(FieldDefinition { name, _eq: eq, bit_spec })
        }
    }
}
