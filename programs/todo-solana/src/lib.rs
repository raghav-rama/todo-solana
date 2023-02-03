use anchor_lang::prelude::*;

declare_id!("5MwohuA1kjtkXfugMqchkyhnUjWZQf9GGWgnHTyW1977");

#[program]
pub mod todo_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initialize!");
        let my_account = &mut ctx.accounts.my_account;
        my_account.todo = String::from("What do u wanna do?");
        my_account.done = false;
        my_account.time = Clock::get().unwrap().unix_timestamp;
        msg!("Todo: {}", my_account.todo);
        msg!("Done?: {}", my_account.done);
        msg!("Time created: {}", my_account.time);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = MyAccount::MAX_SIZE)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>
}

#[account]
pub struct MyAccount {
    pub todo: String,
    pub done: bool,
    pub time: i64,
}

impl MyAccount {
    pub const MAX_SIZE: usize = 4 + 19 + 1 + 8;
}
