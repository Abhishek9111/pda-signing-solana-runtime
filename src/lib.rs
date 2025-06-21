use solana_program::{pubkey::Pubkey,program::invoke_signed,instruction::{AccountMeta, Instruction},account_info::{AccountInfo,next_account_info},entrypoint,entrypoint::{ProgramResult},system_instruction::create_account};

!entrypoint(process_instruction)

fn process_instruction(
    public_key:&Pubkey,//program id
    accounts:&[AccountInfo],
    instruction_data:&[u8]
) -> ProgramResult {

    let iter  = &mut accounts.iter();
    let pda = next_account_info(iter)?;
    let user_acc = next_account_info(iter)?;
    let system_program = next_account_info(iter)?;

    let seeds = &[user_acc.key.as_ref(),"user-seeds"] //converting to bytes

   let (pda_public_key,bump) = Pubkey::find_program_address(seeds,program_id);
    let ix = create_account(user_acc.key,pda.key,1000000000,8,public_key);

    invoke_signed(ix,[accounts[1],accounts[0],accounts[2]],&[&[seeds,&[bump]]]);

}