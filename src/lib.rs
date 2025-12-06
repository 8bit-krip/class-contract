use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::{create_account, transfer},
    program::invoke_signed,
    sysvar::{rent::Rent, Sysvar},
};


entrypoint!(process_instruction);


pub fn process_instruction(
    program_id: &Pubkey,      
    accounts: &[AccountInfo], 
    _instruction_data: &[u8],  
) -> ProgramResult {

    let iter = &mut accounts.iter();
    let pda = next_account_info(iter)?;          // PDA account
    let user = next_account_info(iter)?;         // User (payer)
    let system_program = next_account_info(iter)?; // System Program

    
    let (pda_pubkey, bump_seed) =
        Pubkey::find_program_address(&[user.key.as_ref(), b"user"], program_id);

    
    if pda_pubkey != *pda.key {
        msg!("Error: Provided PDA does not match derived PDA");
        return Err(ProgramError::InvalidArgument);
    }

    let seeds = &[user.key.as_ref(), b"user"];

    
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(0);

    
    let ix = create_account(
        user.key,
        pda.key,
        lamports,  
        0,          
        program_id, 
    );

    msg!("Creating PDA account");

    invoke_signed(&ix, accounts &[&[seeds,&[bump_seed],]],)?;
    Ok(())
}
