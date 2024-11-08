use proc_macro2::TokenStream;
use quote::quote;
use shank_macro_impl::{instruction::InstructionVariant, syn};

pub(crate) fn generate_context(variant: &InstructionVariant) -> TokenStream {
    // accounts fields
    let struct_fields = variant.accounts.iter().map(|account| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
            if account.optional {
                quote! {
                    pub #account_name: Option<&'a AccountInfo>
                }
            } else {
                quote! {
                    pub #account_name:&'a AccountInfo
                }
            }
        });

    // accounts initialization
    let account_fields = variant.accounts.iter().enumerate().map(|(index, account)| {
            let account_name = syn::parse_str::<syn::Ident>(&account.name).unwrap();
            if account.optional {
                quote! {
                    #account_name: if accounts[#index].key() == &crate::ID { None } else { Some(&accounts[#index]) }
                }
            } else {
                quote! {
                    #account_name: &accounts[#index]
                }
            }
        });

    let expected = variant.accounts.len(); // number of expected accounts
    let name =
        syn::parse_str::<syn::Ident>(&format!("{}Accounts", variant.ident))
            .unwrap();

    quote! {
        pub struct #name<'a> {
            #(#struct_fields,)*
        }
        impl<'a> #name<'a> {
            pub fn context(
                accounts: &'a [AccountInfo]
            ) -> Result<Context<'a, Self>, ProgramError> {
                if accounts.len() < #expected {
                    return Err(ProgramError::NotEnoughAccountKeys);
                }

                Ok(Context {
                    accounts: Self { #(#account_fields,)* },
                    remaining_accounts: &accounts[#expected..],
                })
            }
        }
    }
}
