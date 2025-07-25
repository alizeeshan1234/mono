use crate::errors::RalliError;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + Game::INIT_SPACE,
        seeds = [b"game", game_id.to_le_bytes().as_ref()],
        bump
    )]
    pub game: Account<'info, Game>,

    #[account(
        init,
        payer = creator,
        space = 8 + GameEscrow::INIT_SPACE,
        seeds = [b"escrow", game.key().as_ref()],
        bump
    )]
    pub game_escrow: Account<'info, GameEscrow>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateGame<'info> {
    pub fn create_game(
        &mut self,
        game_id: u64,
        max_users: u8,
        entry_fee: u64,
        bumps: &CreateGameBumps,
    ) -> Result<()> {
        require!(max_users >= 2, RalliError::NotEnoughUsers);
        require!(max_users <= 50, RalliError::GameFull);
        require!(entry_fee > 0, RalliError::InvalidEntryFee);

        let game = &mut self.game;
        let game_escrow = &mut self.game_escrow;
        let clock = Clock::get()?;

        game.set_inner(Game {
            game_id,
            creator: self.creator.key(),
            users: Vec::new(),
            max_users,
            entry_fee,
            status: GameStatus::Open,
            created_at: clock.unix_timestamp,
            locked_at: None,
            bump: bumps.game,
        });

        game_escrow.set_inner(GameEscrow {
            game: game.key(),
            total_amount: 0,
            bump: bumps.game_escrow,
        });

        Ok(())
    }
}
