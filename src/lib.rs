use solana_program::{pubkey::Pubkey,program::invoke_signed,account_info::{AccountInfo,next_account_info},entrypoint,entrypoint::{ProgramResult},system_instruction::create_account};

entrypoint!(process_instruction);

fn process_instruction(
    program_id:&Pubkey,//program id
    accounts:&[AccountInfo],
    instruction_data:&[u8]
) -> ProgramResult {

    let iter  = &mut accounts.iter();
    let user_acc = next_account_info(iter)?;
    let pda = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    let (pda_public_key,bump) = Pubkey::find_program_address(&[b"client1",user_acc.key.as_ref()],program_id);
    let seeds = &[b"client1",user_acc.key.as_ref(),&[bump]]; //converting to bytes

    let ix = create_account(&user_acc.key,&pda.key,1000000000,8,program_id);

    invoke_signed(&ix,accounts,&[seeds]);

    Ok(())
}