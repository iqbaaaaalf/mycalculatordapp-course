use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalculatordapp {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    // context is where the account reside (state)
    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
      let calculator = &mut ctx.accounts.calculator;
      calculator.greeting = init_message;
      Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
  // initialize calculator account in solana
  // payer is the user that sign the transaction
  #[account(init, payer=user, space=264)]
  pub calculator: Account<'info, Calculator>,

  // mark user argument as mutable
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>
}

#[account]
pub struct Calculator {
  pub greeting: String,
  // i64 = integer 64
  pub result: i64,
  pub reminder: i64
}

