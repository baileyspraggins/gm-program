use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define type of state stored in account
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    pub name: String,
}

// Declare abnd export eh program's entrypoint
entrypoint!(process_instruction);

// Programs entrypoints implementation
pub fn process_instruction(
    program_id: &Pubkey,      // Public key of the account
    accounts: &[AccountInfo], // Account that the program is interacting with
    input: &[u8],             // String input data that the program will say Good Morning too
) -> ProgramResult {
    msg!("GM program entrypoint");

    // Iterate through accounts
    let accounts_iter = &mut accounts.iter();

    // Get the account we will be interacting with
    let account = next_account_info(accounts_iter)?;

    // Make sure that the account is owned by the program
    if account.owner != program_id {
        msg!("Greeted account does not ahve the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialzie the input data and store in the greeting account struct
    let input_data = GreetingAccount::try_from_slice(&input).unwrap();

    // Say GM in the program output
    msg!("Good Morning! {}", input_data.name);

    // Serialize the name and store it in the passed account
    input_data.serialize(&mut &mut account.try_borrow_mut_data()?[..])?;

    Ok(())
}
