mod bit_specification;
mod field_definition;
mod instruction;

use {crate::instruction::Instruction, proc_macro::TokenStream, quote::quote, syn::parse_macro_input};

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
