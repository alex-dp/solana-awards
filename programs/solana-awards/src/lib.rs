use anchor_lang::prelude::*;

const VOTE: &str = "vote";
const LIST: &str = "list";
const CANDIDATE: &str = "candidate";

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

    #[account(
        mut,
        seeds = [CANDIDATE.as_bytes(), &preference.piece.key().to_bytes()],
        bump
    )]
    pub preference: Account<'info, CandidateAccount>,

    ///CHECK: x
    #[account(signer, mut)]
    pub user_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AmendVote<'info> {
    #[account(
        seeds = [VOTE.as_bytes(), &user_authority.key.to_bytes()],
        bump
    )]
    pub voter_account: Account<'info, VoterAccount>,

    #[account(
        mut,
        seeds = [CANDIDATE.as_bytes(), &former_preference.piece.key().to_bytes()],
        bump
    )]
    pub former_preference: Account<'info, CandidateAccount>,

    #[account(
        mut,
        seeds = [CANDIDATE.as_bytes(), &new_preference.piece.key().to_bytes()],
        bump
    )]
    pub new_preference: Account<'info, CandidateAccount>,

    ///CHECK: x
    #[account(signer, mut)]
    pub user_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeFirst<'info> {
    #[account(
        init,
        payer = user_authority,
        space = 8 + CandidateAccount::MAX_SIZE,
        seeds = [CANDIDATE.as_bytes(), &piece.key().to_bytes()],
        bump
    )]
    pub candidate: Account<'info, CandidateAccount>,

    #[account(
        init,
        payer = user_authority,
        space = 8 + CandidateList::MAX_SIZE,
        seeds = [LIST.as_bytes()],
        bump
    )]
    pub list: Account<'info, CandidateList>,

    ///CHECK: x
    #[account()]
    pub piece: AccountInfo<'info>,

    ///CHECK: x
    #[account(signer, mut)]
    pub user_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListPiece<'info> {
    #[account(
        init,
        payer = user_authority,
        space = 8 + CandidateAccount::MAX_SIZE,
        seeds = [CANDIDATE.as_bytes(), &piece.key().to_bytes()],
        bump
    )]
    pub candidate: Account<'info, CandidateAccount>,

    #[account(mut)]
    pub list: Account<'info, CandidateList>,

    ///CHECK: x
    #[account()]
    pub piece: AccountInfo<'info>,

    ///CHECK: x
    #[account(signer, mut)]
    pub user_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
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
pub struct CandidateList {
    size: u16
}

impl CandidateList {
    pub const MAX_SIZE: usize = 2;
}

#[account]
pub struct CandidateAccount {
    piece: Pubkey,
    votes: u64,
}

impl CandidateAccount {
    pub const MAX_SIZE: usize = 40;
}