use pinocchio::{account_info::AccountInfo, program_error::ProgramError};
use swig_shank_macro::{ShankContext, ShankInstruction};

pub const ID: [u8; 32] = [42; 32];

#[derive(ShankContext, ShankInstruction)]
pub enum Instruction {
    #[account(0, name = "wallet")]
    #[account(1, optional, name = "optional_account")]
    ReadWallet,
}

#[test]
fn generated_context_uses_pinocchio_accounts_and_errors() {
    assert!(matches!(
        accounts::ReadWalletAccounts::context(&[]),
        Err(ProgramError::NotEnoughAccountKeys)
    ));
}
