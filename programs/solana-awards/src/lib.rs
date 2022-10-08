use anchor_lang::prelude::*;

const VOTE: &str = "vote";
const LIST: &str = "list";
const CANDIDATE: &str = "candidate";

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_awards {
    use super::*;

    pub fn vote(ctx: Context<Vote>) -> Result<()> {
        let voter_account = &mut ctx.accounts.voter_account;
        let preference = &mut ctx.accounts.preference;

        voter_account.preference = preference.piece;
        preference.votes += 1;

        Ok(())
    }

    pub fn amend_vote(ctx: Context<AmendVote>) -> Result<()> {
        let voter_account = &mut ctx.accounts.voter_account;
        let former_preference = &mut ctx.accounts.former_preference;
        let new_preference = &mut ctx.accounts.new_preference;

        voter_account.preference = new_preference.piece;
        former_preference.votes -= 1;
        new_preference.votes += 1;

        Ok(())
    }

    pub fn initialize_list(_ctx: Context<InitializeList>) -> Result<()> {
        Ok(())
    }

    pub fn list_piece(ctx: Context<ListPiece>) -> Result<()> {
        let list = &mut ctx.accounts.list;
        let candidate = &mut ctx.accounts.candidate;

        list.size += 1;

        candidate.votes = 0;
        candidate.piece = *ctx.accounts.piece.key;
        candidate.index = list.size;
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
        bump,
        constraint = voter_account.owner == user_authority.key()
    )]
    pub voter_account: Account<'info, VoterAccount>,

    #[account(
        mut,
        seeds = [CANDIDATE.as_bytes(), &preference.index.to_be_bytes()],
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
        bump,
        constraint = voter_account.owner == user_authority.key()
    )]
    pub voter_account: Account<'info, VoterAccount>,

    #[account(
        mut,
        seeds = [CANDIDATE.as_bytes(), &former_preference.index.to_be_bytes()],
        bump,
        constraint = former_preference.piece != voter_account.preference
    )]
    pub former_preference: Account<'info, CandidateAccount>,

    #[account(
        mut,
        seeds = [CANDIDATE.as_bytes(), &new_preference.index.to_be_bytes()],
        bump,
        constraint = former_preference.index != new_preference.index
    )]
    pub new_preference: Account<'info, CandidateAccount>,

    ///CHECK: x
    #[account(signer, mut)]
    pub user_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeList<'info> {

    #[account(
        init,
        payer = user_authority,
        space = 8 + CandidateList::MAX_SIZE,
        seeds = [LIST.as_bytes()],
        bump
    )]
    pub list: Account<'info, CandidateList>,

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
        seeds = [CANDIDATE.as_bytes(), &list.size.to_be_bytes()],
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
    // NOTE: is a token PK, not CandidateAccount PK
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
    // NOTE: is token PK
    piece: Pubkey,
    votes: u64,
    index: u16
}

impl CandidateAccount {
    pub const MAX_SIZE: usize = 42;
}