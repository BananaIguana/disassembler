use {
    crate::{bit_specification::BitSpecification, field_definition::FieldDefinition},
    proc_macro2::{Ident, TokenStream},
    quote::quote,
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

impl Instruction
{
    pub fn process_struct_fields(fields: &Punctuated<FieldDefinition, Comma>) -> Vec<TokenStream>
    {
        fields
            .iter()
            .flat_map(|f| {
                let ident = &f.name;
                let fname = ident.to_string();
                match &f.bit_spec
                {
                    BitSpecification::Mask { .. } => vec![quote! { pub mask: u32, }, quote! { pub #ident: u32, }],
                    BitSpecification::Single(_) =>
                    {
                        if fname.starts_with('r') || fname.starts_with('R')
                        {
                            vec![quote! { pub #ident: Register, }]
                        }
                        else
                        {
                            vec![quote! { pub #ident: bool, }]
                        }
                    }
                    BitSpecification::Range { start, end } =>
                    {
                        let width = end - start + 1;
                        // Determine integer type or Register
                        if fname.starts_with('r') || fname.starts_with('R')
                        {
                            // registers always u64 into Register
                            vec![quote! { pub #ident: Register, }]
                        }
                        else
                        {
                            let ty = if width <= 8
                            {
                                quote! { u8 }
                            }
                            else if width <= 16
                            {
                                quote! { u16 }
                            }
                            else if width <= 32
                            {
                                quote! { u32 }
                            }
                            else if width <= 64
                            {
                                quote! { u64 }
                            }
                            else
                            {
                                quote! { u128 }
                            };
                            vec![quote! { pub #ident: #ty, }]
                        }
                    }
                }
            })
            .collect::<Vec<_>>()
    }

    /// For example:
    ///
    /// let mask = 0xFF0000FF;
    /// let Rd: Register = (<instruction value> >> 12).into();
    /// let sf = (<instruction value> >> 6) != 0;
    /// let imm12: u16 = ((<instruction value> >> 20 & mask)) as u16;
    pub fn process_from(fields: &Punctuated<FieldDefinition, Comma>) -> Vec<TokenStream>
    {
        fields
            .iter()
            .flat_map(|f| {
                let ident = &f.name;
                let fname = ident.to_string();
                match f.bit_spec
                {
                    BitSpecification::Mask { mask, check, .. } =>
                    {
                        vec![quote! { let mask = #mask; }, quote! { let #ident = #check; }]
                    }
                    BitSpecification::Single(bit) =>
                    {
                        let shift = bit;
                        let mask = 1u32 << shift;
                        if fname.starts_with('r') || fname.starts_with('R')
                        {
                            vec![quote! { let #ident: Register = (((instruction & #mask) >> #shift) as u64).into(); }]
                        }
                        else
                        {
                            vec![quote! { let #ident = ((instruction & #mask) >> #shift) != 0; }]
                        }
                    }
                    BitSpecification::Range { start, end } =>
                    {
                        let width = end - start + 1;
                        let mask_bits = (1u128 << width) - 1;
                        let mask = quote! { (#mask_bits as u32) };
                        if fname.starts_with('r') || fname.starts_with('R')
                        {
                            vec![quote! { let #ident: Register = (((instruction >> #start) & #mask) as u64).into(); }]
                        }
                        else
                        {
                            let int_ty = if width <= 8
                            {
                                quote! { u8 }
                            }
                            else if width <= 16
                            {
                                quote! { u16 }
                            }
                            else if width <= 32
                            {
                                quote! { u32 }
                            }
                            else if width <= 64
                            {
                                quote! { u64 }
                            }
                            else
                            {
                                quote! { u128 }
                            };
                            vec![quote! { let #ident: #int_ty = (((instruction >> #start) & #mask) as #int_ty); }]
                        }
                    }
                }
            })
            .collect::<Vec<_>>()
    }

    /// For example:
    ///
    /// sf,
    /// S,
    /// Rn,
    pub fn process_initialisers(fields: &Punctuated<FieldDefinition, Comma>) -> Vec<TokenStream>
    {
        fields
            .iter()
            .flat_map(|f| {
                let ident = &f.name;
                match f.bit_spec
                {
                    BitSpecification::Mask { .. } => vec![quote! { mask, }, quote! { #ident, }],
                    _ => vec![quote! { #ident, }],
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn process_debug(fields: &Punctuated<FieldDefinition, Comma>) -> Vec<TokenStream>
    {
        fields
            .iter()
            .flat_map(|f| {
                let ident = &f.name;
                let fname = ident.to_string();
                match f.bit_spec
                {
                    BitSpecification::Mask { .. } => vec![
                        quote! { .field("mask", &format_args!("{:#x}", self.mask)) },
                        quote! { .field(stringify!(#ident), &format_args!("{:#x}", self.#ident)) },
                    ],
                    BitSpecification::Single(_) =>
                    {
                        if fname.starts_with('r') || fname.starts_with('R')
                        {
                            vec![quote! { .field(stringify!(#ident), &self.#ident) }]
                        }
                        else
                        {
                            vec![quote! { .field(stringify!(#ident), &self.#ident) }]
                        }
                    }
                    BitSpecification::Range { .. } =>
                    {
                        if fname.starts_with('r') || fname.starts_with('R')
                        {
                            vec![quote! { .field(stringify!(#ident), &self.#ident) }]
                        }
                        else
                        {
                            vec![quote! { .field(stringify!(#ident), &format_args!("{:#x}", self.#ident)) }]
                        }
                    }
                }
            })
            .collect::<Vec<_>>()
    }
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
