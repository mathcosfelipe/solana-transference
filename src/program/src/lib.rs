use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    system_instruction::transfer,
    program::invoke
};

// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of transactions
    pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    let account = next_account_info(accounts_iter)?;
    let account_receiver = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let transference = transfer(account.key, account_receiver.key, 150000);
    invoke(&transference, &[account.clone(), account_receiver.clone(), system_program.clone()])?;

    Ok(())
}