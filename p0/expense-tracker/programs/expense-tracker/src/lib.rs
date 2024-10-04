use anchor_lang::prelude::*;

declare_id!("4hNPjuksKiqAh1bDFXrutEzuy3dgkit51tzGRtNUeWWs");

#[program]
pub mod expense_tracker {
    use super::*;

    pub fn initialize_expense(
        ctx: Context<InitializeExpense>,
        id: u64,
        merchant_name: String,
        amount: u64,
    ) -> Result<()> {
        let expense_tracker = &mut ctx.accounts.expense_account;
        expense_tracker.id = id;
        expense_tracker.amount = amount;
        expense_tracker.owner = *ctx.accounts.signer.key;
        expense_tracker.merchant_name = merchant_name;

        Ok(())
    }

    pub fn modify_expense(
        ctx: Context<ModifyExpense>,
        _id: u64,
        merchant_name: String,
        amount: u64,
    ) -> Result<()> {
        let expense_tracker = &mut ctx.accounts.expense_account;
        expense_tracker.amount = amount;
        expense_tracker.merchant_name = merchant_name;

        Ok(())
    }

    pub fn delete_expense(_ctx: Context<DeleteExpense>, _id: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(
  id: u64
)]
pub struct InitializeExpense<'i> {
    #[account(mut)]
    pub signer: Signer<'i>,

    #[account(
      init, // IMPORTANT!
      payer = signer, // IMPORTANT! 
      space = 8 + 8 + 32 + (4 + 12)+ 8 + 1, // IMPORTANT! discriminator + id + pubkey + (merchant name) + amount + bump
      seeds = [b"expense", signer.key().as_ref(), id.to_le_bytes().as_ref()], // IMPORTANT!
      bump
    )]
    pub expense_account: Account<'i, ExpenseAccount>,

    pub system_program: Program<'i, System>,
}

#[derive(Accounts)]
#[instruction(
  id: u64
)]
pub struct ModifyExpense<'i> {
    #[account(mut)]
    pub signer: Signer<'i>,

    #[account(
      mut,
      seeds = [b"expense", signer.key().as_ref(), id.to_le_bytes().as_ref()],
      bump
    )]
    pub expense_account: Account<'i, ExpenseAccount>,

    pub system_program: Program<'i, System>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct DeleteExpense<'i> {
    #[account(mut)]
    pub signer: Signer<'i>,

    #[account(
    mut,
    close = signer, // IMPORTANT!
    seeds = [b"expense", signer.key().as_ref(), id.to_le_bytes().as_ref()],
    bump
  )]
    pub expense_account: Account<'i, ExpenseAccount>,

    pub system_program: Program<'i, System>,
}

#[account]
pub struct ExpenseAccount {
    id: u64,
    owner: Pubkey,
    merchant_name: String,
    amount: u64,
}
