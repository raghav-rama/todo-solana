use anchor_lang::prelude::*;

declare_id!("5MwohuA1kjtkXfugMqchkyhnUjWZQf9GGWgnHTyW1977");

#[program]
pub mod todo_solana {
    use super::*;

    pub fn initialize(ctx: Context<InitializeTodo>) -> Result<()> {
        let pubkey = ctx.accounts.todo_item.to_account_info().key();

        let head = &mut ctx.accounts.genesis_todo_account;
        let current = &mut ctx.accounts.current;
        let todo_item = &mut ctx.accounts.todo_item;

        todo_item.todo = String::from("Genesis Todo");
        todo_item.done = false;
        todo_item.time = Clock::get()?.unix_timestamp;
        todo_item.next = Pubkey::default();
        
        current.current = pubkey;
        head.head = pubkey; 
        Ok(())
    }

    pub fn add_todo(ctx: Context<AddTodo>, todo: String) -> Result<()> {
        let pubkey = ctx.accounts.new_todo_item.to_account_info().key();
        
        let new_todo_item = &mut ctx.accounts.new_todo_item;
        let current  = &mut ctx.accounts.current;
        let previous = &mut ctx.accounts.previous;

        new_todo_item.todo = todo;
        new_todo_item.done = false;
        new_todo_item.time = Clock::get()?.unix_timestamp;
        new_todo_item.next = Pubkey::default();

        current.current = pubkey;
        previous.next = pubkey;        
        Ok(())
    }
}


#[derive(Accounts)]
pub struct InitializeTodo<'info> {
    #[account(init, payer = user, space = TodoItem::MAX_SIZE)]
    pub todo_item: Account<'info, TodoItem>,

    #[account(init, payer = user, space = CurrentTodoItem::MAX_SIZE)]
    pub current: Account<'info, CurrentTodoItem>,

    #[account(init, payer = user, space = Head::MAX_SIZE)]
    pub genesis_todo_account: Account<'info, Head>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddTodo<'info> {
    #[account(init, payer = user, space = TodoItem::MAX_SIZE)]
    pub new_todo_item: Account<'info, TodoItem>,

    #[account(mut)]
    pub previous: Account<'info, TodoItem>,

    #[account(mut)]
    pub current: Account<'info, CurrentTodoItem>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// #[derive(AnchorDeserialize, AnchorSerialize, Clone)]
#[account]
pub struct TodoItem {
    pub todo: String,
    pub done: bool,
    pub time: i64,
    pub next: Pubkey,
}

#[account]
pub struct Head {
    pub head: Pubkey,
}

#[account]
pub struct CurrentTodoItem {
    pub current: Pubkey,
}

impl TodoItem {
    pub const MAX_SIZE: usize = 8 + 4 + 200 + 1 + 8 + 8; // Todo String length is 200
}

impl Head {
    pub const MAX_SIZE: usize = 8 + 32;
}

impl CurrentTodoItem {
    pub const MAX_SIZE: usize = 8 + 32;
}


