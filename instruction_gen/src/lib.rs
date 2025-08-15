mod bit_specification;
mod field_definition;
mod instruction;

use {
    crate::{bit_specification::BitSpecification, instruction::Instruction},
    proc_macro::TokenStream,
    proc_macro2::Literal,
    quote::quote,
    syn::{Ident, parse_macro_input},
};

#[proc_macro]
pub fn make_instruction(input: TokenStream) -> TokenStream
{
    let Instruction { name, fields, .. } = parse_macro_input!(input as Instruction);

    // Generate struct fields, inferring Register for names starting with 'r' or 'R'
    let struct_fields = Instruction::process_struct_fields(&fields);

    // Extraction logic
    let extractors = Instruction::process_from(&fields);

    // let combined = extractors
    //     .iter()
    //     .map(|ts| ts.to_string())
    //     .collect::<Vec<_>>()
    //     .join("\n\n");
    //
    // // Debug output during build
    // println!("process_from:\n{}\n\n", combined);

    // Initialize struct fields
    let inits = Instruction::process_initialisers(&fields);

    // let combined = inits
    //     .iter()
    //     .map(|ts| ts.to_string())
    //     .collect::<Vec<_>>()
    //     .join("\n\n");
    //
    // // Debug output during build
    // println!("process_initialisers:\n{}\n\n", combined);

    // Custom Debug impl: hex formatting for ranges, mask, and registers
    let debug_fields = Instruction::process_debug(&fields);

    // let combined = debug_fields
    //     .iter()
    //     .map(|ts| ts.to_string())
    //     .collect::<Vec<_>>()
    //     .join("\n\n");
    //
    // // Debug output during build
    // println!("process_debug:\n{}\n\n", combined);

    // Build Display impl if we can find Rd, Rn, and an immediate field
    let mnemonic_str = name.to_string().to_lowercase();
    let mnemonic_lit = Literal::string(&mnemonic_str);

    let rd_ident = fields
        .iter()
        .find(|f| f.name.to_string() == "Rd")
        .map(|f| f.name.clone());
    let rn_ident = fields
        .iter()
        .find(|f| f.name.to_string() == "Rn")
        .map(|f| f.name.clone());
    let imm_ident = fields
        .iter()
        .find(|f| f.name.to_string().to_lowercase().starts_with("imm"))
        .map(|f| f.name.clone());
    let sh_ident = fields
        .iter()
        .find(|f| f.name.to_string() == "sh")
        .map(|f| f.name.clone());

    let display_impl = if let (Some(rd), Some(rn), Some(imm)) = (rd_ident, rn_ident, imm_ident)
    {
        if let Some(sh) = sh_ident
        {
            quote! {
                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        if self.#sh {
                            write!(f, "{} {}, {}, {:#X}, LSL #12", #mnemonic_lit, self.#rd, self.#rn, self.#imm)
                        } else {
                            write!(f, "{} {}, {}, {:#X}", #mnemonic_lit, self.#rd, self.#rn, self.#imm)
                        }
                    }
                }
            }
        }
        else
        {
            quote! {
                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{} {}, {}, {:#X}", #mnemonic_lit, self.#rd, self.#rn, self.#imm)
                    }
                }
            }
        }
    }
    else
    {
        quote! {}
    };

    // Build an associated function returning the mask/check pattern as a string
    let mask_pattern = fields.iter().find_map(|f| {
        if let BitSpecification::Mask { pattern, .. } = &f.bit_spec
        {
            Some(Literal::string(pattern))
        }
        else
        {
            None
        }
    });

    let mask_accessor = if let Some(pat) = mask_pattern
    {
        quote! {
            impl #name {
                /// Returns the instruction's check/mask pattern string (with '?' as wildcards)
                pub fn instruction_mask() -> &'static str {
                    #pat
                }
            }
        }
    }
    else
    {
        quote! {}
    };

    // Associated numeric constants + a const-time matcher
    let mask_check_vals = fields.iter().find_map(|f| {
        if let BitSpecification::Mask { mask, check, .. } = f.bit_spec
        {
            Some((mask, check))
        }
        else
        {
            None
        }
    });

    let numeric_impl_consts = if let Some((mask_u32, check_u32)) = mask_check_vals
    {
        quote! {
            impl #name {
                pub const MASK: u32 = #mask_u32;
                pub const CHECK: u32 = #check_u32;
                #[inline(always)]
                pub const fn matches(instruction: u32) -> bool {
                    (instruction & Self::MASK) == Self::CHECK
                }
            }
        }
    }
    else
    {
        quote! {}
    };

    let expanded = quote! {

        #[allow(non_camel_case_types)]
        #[allow(clippy::upper_case_acronyms)]
        pub struct #name
        {
            #(#struct_fields)*
        }

        impl #name
        {
            pub fn from(instruction: u32) -> Self
            {
                #(#extractors)*
                Self { #(#inits)* }
            }
        }

        impl std::fmt::Debug for #name
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
            {
                f.debug_struct(stringify!(#name))
                 #(#debug_fields)*
                 .finish()
            }
        }

        #mask_accessor

        #numeric_impl_consts

        #display_impl
    };

    TokenStream::from(expanded)
}
