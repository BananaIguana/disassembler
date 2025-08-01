use {
    crate::field_definition::FieldDefinition,
    proc_macro2::Ident,
    syn::{
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        token::Comma,
    },
};

pub struct Instruction
{
    pub name: Ident,
    _comma: Comma,
    pub fields: Punctuated<FieldDefinition, Comma>,
}

impl Parse for Instruction
{
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        let name = input.parse()?;
        let comma = input.parse()?;
        let fields = Punctuated::parse_terminated(input)?;
        Ok(Instruction {
            name,
            _comma: comma,
            fields,
        })
    }
}
