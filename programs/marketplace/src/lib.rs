use anchor_lang::prelude::*;

declare_id!("7pLE2CQhUnfyNHj4eF6dXkkY4eu3wSs1rMkb2gRHZhs2");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
