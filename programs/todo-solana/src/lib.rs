use anchor_lang::prelude::*;

declare_id!("5MwohuA1kjtkXfugMqchkyhnUjWZQf9GGWgnHTyW1977");

#[program]
pub mod todo_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
