use proc_macro2::TokenStream;
use quote::quote;
use shank_macro_impl::{
    builder::BuilderVariant,
    instruction::InstructionVariantFields,
    syn::{parse_str, Expr, ExprPath, Ident},
};
use std::collections::HashMap;

pub(crate) fn generate_builders(
    item: &Ident,
    variant: &BuilderVariant,
) -> TokenStream {
    

    let field_names: Vec<Ident> = match &variant.field_tys {
        InstructionVariantFields::Named(field_tys) => field_tys
            .iter()
            .map(|(name, _)| parse_str::<Ident>(name).unwrap())
            .collect(),
        InstructionVariantFields::Unnamed(field_tys) => field_tys
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                parse_str::<Ident>(&format!(
                    "args{}",
                    if idx == 0 {
                        String::new()
                    } else {
                        idx.to_string()
                    }
                ))
                .unwrap()
            })
            .collect(),
    };

    // instruction struct

    // accounts
    let struct_accounts = variant.accounts.iter().map(|account| {
        let account_name = parse_str::<Ident>(&account.name).unwrap();
        if account.optional {
            quote! {
                pub #account_name: Option<Pubkey>
            }
        } else {
            quote! {
                pub #account_name: Pubkey
            }
        }
    });

    // optional signers
    let struct_optional_signers = variant
        .accounts
        .iter()
        .filter(|account| account.optional_signer)
        .map(|account| {
            let optional_signer =
                parse_str::<Ident>(&format!("{}_signer", account.name))
                    .unwrap();
            quote! {
                pub #optional_signer: bool
            }
        });

    // args (builder)
    let struct_builder_args = variant.arguments.iter().map(|argument| {
        let ident_ty = parse_str::<Ident>(&argument.ty).unwrap();
        let arg_ty = if let Some(genetic_ty) = &argument.generic_ty {
            let arg_generic_ty = parse_str::<Ident>(genetic_ty).unwrap();
            quote! { #ident_ty<#arg_generic_ty> }
        } else {
            quote! { #ident_ty }
        };
        let arg_name = parse_str::<Ident>(&argument.name).unwrap();

        quote! {
            pub #arg_name: #arg_ty
        }
    });

    // builder struct

    // accounts
    let builder_accounts = variant.accounts.iter().map(|account| {
        let account_name = parse_str::<Ident>(&account.name).unwrap();
        quote! {
            pub #account_name: Option<Pubkey>
        }
    });

    // optional signers
    let builder_optional_signers = variant
        .accounts
        .iter()
        .filter(|account| account.optional_signer)
        .map(|account| {
            let optional_signer =
                parse_str::<Ident>(&format!("{}_signer", account.name))
                    .unwrap();
            quote! {
                pub #optional_signer: bool
            }
        });

    // accounts initialization
    let builder_initialize_accounts = variant.accounts.iter().map(|account| {
        let account_name = parse_str::<Ident>(&account.name).unwrap();
        quote! {
            #account_name: None
        }
    });

    // optional signers initialization
    let builder_initialize_optional_signers = variant
        .accounts
        .iter()
        .filter(|account| account.optional_signer)
        .map(|account| {
            let optional_signer =
                parse_str::<Ident>(&format!("{}_signer", account.name))
                    .unwrap();
            quote! {
                #optional_signer: false
            }
        });

    // args (builder)
    let builder_args = variant.arguments.iter().map(|argument| {
        let ident_ty = parse_str::<Ident>(&argument.ty).unwrap();
        let arg_ty = if let Some(genetic_ty) = &argument.generic_ty {
            let arg_generic_ty = parse_str::<Ident>(genetic_ty).unwrap();
            quote! { #ident_ty<#arg_generic_ty> }
        } else {
            quote! { #ident_ty }
        };
        let arg_name = parse_str::<Ident>(&argument.name).unwrap();

        quote! {
            pub #arg_name: Option<#arg_ty>
        }
    });

    // args initialization
    let builder_initialize_args = variant.arguments.iter().map(|argument| {
        let arg_name = parse_str::<Ident>(&argument.name).unwrap();
        quote! {
            #arg_name: None
        }
    });

    // account setter methods
    let builder_accounts_methods = variant.accounts.iter().map(|account| {
            let account_name = parse_str::<Ident>(&account.name).unwrap();

            if account.optional_signer {
                let optional_signer = parse_str::<Ident>(&format!("{}_signer", account.name)).unwrap();
                quote! {
                    pub fn #account_name(&mut self, #account_name: Pubkey, signer: bool) -> &mut Self {
                        self.#account_name = Some(#account_name);
                        self.#optional_signer = signer;
                        self
                    }
                }
            } else {
            quote! {
                pub fn #account_name(&mut self, #account_name: Pubkey) -> &mut Self {
                    self.#account_name = Some(#account_name);
                    self
                }
            }
        }
});

    // args (builder) setter methods
    let builder_args_methods = variant.arguments.iter().map(|argument| {
        let ident_ty = parse_str::<Ident>(&argument.ty).unwrap();
        let arg_ty = if let Some(genetic_ty) = &argument.generic_ty {
            let arg_generic_ty = parse_str::<Ident>(genetic_ty).unwrap();
            quote! { #ident_ty<#arg_generic_ty> }
        } else {
            quote! { #ident_ty }
        };
        let arg_name = parse_str::<Ident>(&argument.name).unwrap();

        quote! {
            pub fn #arg_name(&mut self, #arg_name: #arg_ty) -> &mut Self {
                self.#arg_name = Some(#arg_name);
                self
            }
        }
    });

    // required accounts
    let required_accounts = variant.accounts.iter().map(|account| {
            let account_name = parse_str::<Ident>(&account.name).unwrap();

            if account.optional {
                quote! {
                    #account_name: self.#account_name
                }
            } else {
                quote! {
                    #account_name: self.#account_name.ok_or(concat!(stringify!(#account_name), " is not set"))?
                }
            }
        });

    // required optional signers
    let required_optional_signers = variant
        .accounts
        .iter()
        .filter(|account| account.optional_signer)
        .map(|account| {
            let optional_signer =
                parse_str::<Ident>(&format!("{}_signer", account.name))
                    .unwrap();
            quote! {
                #optional_signer: self.#optional_signer
            }
        });

    // required args (builder)
    let required_args = variant.arguments.iter().map(|argument| {
            let arg_name = parse_str::<Ident>(&argument.name).unwrap();
            quote! {
                #arg_name: self.#arg_name.clone().ok_or(concat!(stringify!(#arg_name), " is not set"))?
            }
        });

    // required args (builder) list
    let args: Vec<TokenStream> = match &variant.field_tys {
        InstructionVariantFields::Named(field_tys) => field_tys
            .iter()
            .enumerate()
            .map(|(idx, (_, ty))| {
                let name = field_names.get(idx).unwrap();
                let ty = &ty.ident;

                quote! { #name: #ty }
            })
            .collect(),
        InstructionVariantFields::Unnamed(field_tys) => field_tys
            .iter()
            .enumerate()
            .map(|(idx, ty)| {
                let name = field_names.get(idx).unwrap();
                let ty = &ty.ident;

                quote! { #name: #ty }
            })
            .collect(),
    };

    // instruction args
    let instruction_args: Vec<TokenStream> = match &variant.field_tys {
        InstructionVariantFields::Named(field_tys) => field_tys
            .iter()
            .enumerate()
            .map(|(idx, (_, ty))| {
                let name = field_names.get(idx).unwrap();
                let ty = &ty.ident;

                quote! { pub #name: #ty }
            })
            .collect(),
        InstructionVariantFields::Unnamed(field_tys) => field_tys
            .iter()
            .enumerate()
            .map(|(idx, ty)| {
                let name = field_names.get(idx).unwrap();
                let ty = &ty.ident;

                quote! { pub #name: #ty }
            })
            .collect(),
    };

    // required instruction args
    let required_instruction_args: Vec<TokenStream> = match &variant.field_tys {
        InstructionVariantFields::Named(field_tys) => field_tys
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                let name = field_names.get(idx).unwrap();
                quote! { #name }
            })
            .collect(),
        InstructionVariantFields::Unnamed(field_tys) => field_tys
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                let name = field_names.get(idx).unwrap();
                quote! { #name }
            })
            .collect(),
    };

    // account metas
    let account_metas: Vec<TokenStream> = variant.accounts.iter().map(|account| {
        let account_name = parse_str::<Ident>(&account.name).unwrap();
        let signer = if account.optional_signer {
            parse_str::<Expr>(&format!("self.{}_signer", account.name)).unwrap()
        } else {
            parse_str::<Expr>(&format!("{}", account.signer)).unwrap()
        };

        if account.optional {
            if account.writable {
                quote! {
                    if let Some(#account_name) = self.#account_name {
                        AccountMeta::new(#account_name, #signer)
                    } else {
                        AccountMeta::new_readonly(crate::ID, false)
                    }
                }
            } else if account.signer {
                quote! {
                    if let Some(#account_name) = self.#account_name {
                        AccountMeta::new_readonly(#account_name, #signer)
                    } else {
                        AccountMeta::new_readonly(crate::ID, false)
                    }
                }
            } else {
                quote!{
                    AccountMeta::new_readonly(self.#account_name.unwrap_or(crate::ID), false)
                }
            }
        } else if account.writable {
            quote! {
                AccountMeta::new(self.#account_name, #signer)
            }
        } else {
            quote!{
                AccountMeta::new_readonly(self.#account_name, #signer)
            }
        }
    }).collect();

    // builder name
    let name = &variant.ident;
    let builder_name = parse_str::<Ident>(&format!("{}Builder", name)).unwrap();

    // instruction args list
    let struct_instruction_args: Vec<TokenStream> = match &variant.field_tys {
        InstructionVariantFields::Named(field_tys) => field_tys
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                let name = field_names.get(idx).unwrap();
                quote! { self.#name.clone() }
            })
            .collect(),
        InstructionVariantFields::Unnamed(field_tys) => field_tys
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                let name = field_names.get(idx).unwrap();
                quote! { self.#name.clone() }
            })
            .collect(),
    };

    let instruction_data = if struct_instruction_args.is_empty() {
        quote! {
            #item::#name.try_to_vec().unwrap()
        }
    } else {
        quote! {
            #item::#name(#(#struct_instruction_args,)*).try_to_vec().unwrap()
        }
    };

    // default instruction builder (only generated if the instruction builder does
    // not have custom arguments)
    let default_instruction_builder = if variant.arguments.is_empty() {
        quote! {
            impl InstructionBuilder for #name {
                fn instruction(&self) -> Instruction {
                    Instruction {
                        program_id: crate::ID,
                        accounts: vec![
                            #(#account_metas,)*
                        ],
                        data: #instruction_data,
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        pub struct #name {
            #(#struct_accounts,)*
            #(#struct_optional_signers,)*
            #(#instruction_args,)*
            #(#struct_builder_args,)*
        }

        #default_instruction_builder

        pub struct #builder_name {
            #(#builder_accounts,)*
            #(#builder_optional_signers,)*
            #(#builder_args,)*
        }

        impl #builder_name {
            pub fn new() -> Box<#builder_name> {
                Box::new(#builder_name {
                    #(#builder_initialize_accounts,)*
                    #(#builder_initialize_optional_signers,)*
                    #(#builder_initialize_args,)*
                })
            }

            #(#builder_accounts_methods)*
            #(#builder_args_methods)*

            pub fn build(&mut self, #(#args,)*) -> Result<Box<#name>, Box<dyn std::error::Error>> {
                Ok(Box::new(#name {
                    #(#required_accounts,)*
                    #(#required_optional_signers,)*
                    #(#required_instruction_args,)*
                    #(#required_args,)*
                }))
            }
        }
    }
}
