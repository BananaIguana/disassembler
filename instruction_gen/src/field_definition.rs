use {
    crate::bit_specification::BitSpecification,
    proc_macro2::Ident,
    syn::{
        LitInt,
        Token,
        parse::{Parse, ParseStream},
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
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        let name = input.parse()?;
        let eq = input.parse()?;
        let first: LitInt = input.parse()?;

        // Accept either a single bit or an inclusive range using `:`
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
