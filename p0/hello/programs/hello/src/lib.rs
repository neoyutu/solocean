use anchor_lang::prelude::*;

declare_id!("6FGZoNNrsPvEqDJzG2SNbadBQG2DynndjAktf9jzXByC");

#[program]
pub mod hello {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        // Get the current block (slot) number
        let current_slot = Clock::get()?.slot;
        msg!("Current block (slot) number: {}", current_slot);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
