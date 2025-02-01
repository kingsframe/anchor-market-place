use anchor_lang::prelude::*;

declare_id!("7pLE2CQhUnfyNHj4eF6dXkkY4eu3wSs1rMkb2gRHZhs2");

mod state;
mod errors;

mod contexts;
use contexts::*;
use errors::*;
#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;

        Ok(())
    }
}
