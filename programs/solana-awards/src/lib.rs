use anchor_lang::prelude::*;

const VOTE: &str = "vote";
const PIECE: &str = "piece";

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_awards {
    use super::*;

    pub fn vote(ctx: Context<Vote>) -> Result<()> {
        Ok(())
    }

    pub fn amend_vote(ctx: Context<AmendVote>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_first(ctx: Context<InitializeFirst>) -> Result<()> {
        Ok(())
    }

    pub fn list_piece(ctx: Context<ListPiece>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(
        init,
        payer = user_authority,
        space = 8 + VoterAccount::MAX_SIZE,
        seeds = [VOTE.as_bytes(), &user_authority.key.to_bytes()],
        bump
    )]
    pub voter_account: Account<'info, VoterAccount>,

    #[account(mut)]
    pub preference_account: Account<'info, CandidateAccount>,

    #[account(signer, mut)]
    pub user_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AmendVote<'info> {
    #[account(mut)]
    pub voter_account: Account<'info, VoterAccount>,

    #[account(mut)]
    pub former_preference: Account<'info, CandidateAccount>,

    #[account(mut)]
    pub new_preference: Account<'info, CandidateAccount>,

    #[account(signer, mut)]
    pub user_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeFirst<'info> {
    #[account(mut)]
    pub candidate: Account<'info, CandidateAccount>,

    #[account(signer, mut)]
    pub user_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ListPiece<'info> {
    #[account(mut)]
    pub candidate: Account<'info, CandidateAccount>,

    #[account(mut)]
    pub last: Account<'info, CandidateAccount>,

    #[account(signer, mut)]
    pub user_authority: AccountInfo<'info>,
}

#[account]
pub struct VoterAccount {
    owner: Pubkey,
    preference: Pubkey
}

impl VoterAccount {
    pub const MAX_SIZE: usize = 64;
}

#[account]
pub struct CandidateAccount {
    piece: Pubkey,
    votes: u64,
    next: Pubkey
}

impl CandidateAccount {
    pub const MAX_SIZE: usize = 128;
}