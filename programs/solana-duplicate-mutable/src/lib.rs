use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("DDnJgX2bTAPRSmYz75ytAvRAgcxUKVBMRt2dDZwoVUe4");

#[program]
pub mod solana_duplicate_mutable {
    use super::*;

    //function to initialize the game
    pub fn initialize(ctx : Context<Initialize>) -> Result<()>{
        ctx.accounts.new_player.player = ctx.accounts.payer.key();
        ctx.accounts.new_player.choice = None;
        Ok(())
    }

    //function to play the game
    pub fn rock_paper_scissors_shoot_insecure(
        ctx : Context<RockPaperScissorsInsecure>,
        player_one_choice : RockPaperScissors,
        player_two_choice : RockPaperScissors
    )-> Result<()>{
        ctx.accounts.player_one.choice = Some(player_one_choice);
        ctx.accounts.player_two.choice = Some(player_two_choice);
        Ok(())
    }

    //function to play the game -> secure
    pub fn rock_paper_scissors_shoot_secure(
        ctx : Context<RockPaperScissorsSecure>,
        player_one_choice : RockPaperScissors,
        player_two_choice : RockPaperScissors
    )-> Result<()>{
        ctx.accounts.player_one.choice = Some(player_one_choice);
        ctx.accounts.player_two.choice = Some(player_two_choice);
        Ok(())
    }
}

//struct for the initialize instruction
#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 8
    )]
    pub new_player : Account<'info, PlayerState>,

    #[account(mut)]
    pub payer : Signer<'info>,
    pub system_program : Program<'info, System>
}

//struct for the rock paper scissors instruction -> insecure
#[derive(Accounts)]
pub struct RockPaperScissorsInsecure<'info>{
    #[account(mut)]
    pub player_one : Account<'info, PlayerState>,

    #[account(mut)]
    pub player_two : Account<'info, PlayerState>,
}

//struct for the rock paper scissors instruction -> secure
#[derive(Accounts)]
pub struct RockPaperScissorsSecure<'info>{
    #[account(
        mut,
        constraint = player_one.key() != player_two.key()
    )]

    //taking the two players using the PlayerState struct
    pub player_one: Account<'info, PlayerState>,
    #[account(mut)]
    pub player_two: Account<'info, PlayerState>,
}

//struct for the state of the player
#[account]
pub struct PlayerState{
    player : Pubkey,
    choice : Option<RockPaperScissors>
}

//enum for type of play the user does
#[derive(Clone, Copy, BorshDeserialize, BorshSerialize)]
pub enum RockPaperScissors{
    Rock,
    Paper,
    Scissors
}

