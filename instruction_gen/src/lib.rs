mod bit_specification;
mod field_definition;
mod instruction;

use {
    crate::{bit_specification::BitSpecification, instruction::Instruction},
    proc_macro::TokenStream,
    quote::quote,
    syn::{Ident, parse_macro_input},
};

#[proc_macro]
pub fn make_instruction(input: TokenStream) -> TokenStream
{
    let Instruction { name, fields, .. } = parse_macro_input!(input as Instruction);

    // Generate struct fields, inferring Register for names starting with 'r' or 'R'
    let struct_fields = fields
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
        .collect::<Vec<_>>();

    // Extraction logic
    let extractors = fields
        .iter()
        .flat_map(|f| {
            let ident = &f.name;
            let fname = ident.to_string();
            match f.bit_spec
            {
                BitSpecification::Mask { mask, check, .. } => vec![quote! { let mask = #mask; }, quote! { let #ident = #check; }],
                BitSpecification::Single(bit) =>
                {
                    let shift = bit;
                    let mask = 1u32 << shift;
                    if fname.starts_with('r') || fname.starts_with('R')
                    {
                        vec![quote! { let #ident = (((instruction & #mask) >> #shift) as u64).into(); }]
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
                        vec![quote! { let #ident = (((instruction >> #start) & #mask) as u64).into(); }]
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
                        vec![quote! { let #ident = (((instruction >> #start) & #mask) as #int_ty); }]
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    // Initialize struct fields
    let inits = fields
        .iter()
        .flat_map(|f| {
            let ident = &f.name;
            match f.bit_spec
            {
                BitSpecification::Mask { .. } => vec![quote! { mask, }, quote! { #ident, }],
                _ => vec![quote! { #ident, }],
            }
        })
        .collect::<Vec<_>>();

    // Custom Debug impl: hex formatting for ranges, mask, and registers
    let debug_fields = fields
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
        .collect::<Vec<_>>();

    let expanded = quote! {
        #[allow(non_camel_case_types)]
        #[allow(clippy::upper_case_acronyms)]
        pub struct #name {
            #(#struct_fields)*
        }

        impl #name {
            pub fn from(instruction: u32) -> Self {
                #(#extractors)*
                Self { #(#inits)* }
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#name))
                 #(#debug_fields)*
                 .finish()
            }
        }
    };

    TokenStream::from(expanded)
}
