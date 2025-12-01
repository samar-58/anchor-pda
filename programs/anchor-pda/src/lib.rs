use anchor_lang::prelude::*;

declare_id!("FhAa7amenGbqaDkipL9oECcx7RCPJ9rAoxbdKQNNdbqm");

#[program]
pub mod anchor_pda {
    use super::*;

    pub fn initialize(ctx: Context<CreateAccount>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAccount<'info>{
#[account(mut)]
pub signer : Signer<'info>,

#[account(
    init,
    payer = signer,
    space = 8 + 32 + 8 + 1,
    seeds = [b"client1", signer.key().as_ref()],
    bump
)]

pub pda_account: Account<'info,StakeAccount>,

pub system_program : Program<'info,System>
}

#[account]
pub struct StakeAccount {
num:u32
}
