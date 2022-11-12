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

        voter_account.preference = preference.index;
        preference.votes += 1;

        Ok(())
    }

    // TODO rewrite
    // pub fn amend_vote(ctx: Context<AmendVote>) -> Result<()> {
    //     let voter_account = &mut ctx.accounts.voter_account;
    //     let former_preference = &mut ctx.accounts.former_preference;
    //     let new_preference = &mut ctx.accounts.new_preference;
    //
    //     voter_account.preference = new_preference.piece;
    //     former_preference.votes -= 1;
    //     new_preference.votes += 1;
    //
    //     Ok(())
    // }

    pub fn initialize_list(ctx: Context<InitializeList>, period: u16) -> Result<()> {
        let list = &mut ctx.accounts.list;
        list.period = period;

        Ok(())
    }

    pub fn list_project(ctx: Context<ListProject>, author: String, project: String) -> Result<()> {
        let list = &mut ctx.accounts.list;
        let candidate = &mut ctx.accounts.candidate;

        list.size += 1;

        candidate.votes = 0;
        candidate.author = author;
        candidate.project = project;
        candidate.index = list.size;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(period: u16)]
pub struct Vote<'info> {
    #[account(
        init,
        payer = user_authority,
        space = 8 + VoterAccount::MAX_SIZE,
        seeds = [VOTE.as_bytes(), &user_authority.key.to_bytes(), &period.to_be_bytes()],
        bump,
        constraint = voter_account.owner == user_authority.key() && voter_account.period == period
    )]
    pub voter_account: Account<'info, VoterAccount>,

    #[account(
        mut,
        seeds = [CANDIDATE.as_bytes(), &preference.index.to_be_bytes(), &period.to_be_bytes()],
        bump,
        constraint = preference.period == period
    )]
    pub preference: Account<'info, CandidateAccount>,

    ///CHECK: x
    #[account(signer, mut)]
    pub user_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

// TODO rewrite entirely, not essential
// #[derive(Accounts)]
// pub struct AmendVote<'info> {
//     #[account(
//         seeds = [VOTE.as_bytes(), &user_authority.key.to_bytes()],
//         bump,
//         constraint = voter_account.owner == user_authority.key()
//     )]
//     pub voter_account: Account<'info, VoterAccount>,
//
//     #[account(
//         mut,
//         seeds = [CANDIDATE.as_bytes(), &former_preference.index.to_be_bytes()],
//         bump,
//         constraint = former_preference.piece != voter_account.preference
//     )]
//     pub former_preference: Account<'info, CandidateAccount>,
//
//     #[account(
//         mut,
//         seeds = [CANDIDATE.as_bytes(), &new_preference.index.to_be_bytes()],
//         bump,
//         constraint = former_preference.index != new_preference.index
//     )]
//     pub new_preference: Account<'info, CandidateAccount>,
//
//     ///CHECK: x
//     #[account(signer, mut)]
//     pub user_authority: AccountInfo<'info>,
// }

#[derive(Accounts)]
#[instruction(period: u16)]
pub struct InitializeList<'info> {

    #[account(
        init,
        payer = user_authority,
        space = 8 + CandidateList::MAX_SIZE,
        seeds = [LIST.as_bytes(), &period.to_be_bytes()],
        bump
    )]
    pub list: Account<'info, CandidateList>,

    ///CHECK: x
    #[account(signer, mut)]
    pub user_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(period: u16)]
pub struct ListProject<'info> {

    #[account(
        init,
        payer = user_authority,
        space = 8 + CandidateAccount::MAX_SIZE,
        seeds = [CANDIDATE.as_bytes(), &list.size.to_be_bytes(), &period.to_be_bytes()],
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

// seeds: VOTE, PK bytes
#[account]
pub struct VoterAccount {
    owner: Pubkey,
    // NOTE: is index from candidate account
    preference: u16,
    period: u16
}

impl VoterAccount {
    pub const MAX_SIZE: usize = 64; // TODO recount
}

// seeds: LIST, period
#[account]
pub struct CandidateList {
    size: u16,
    period: u16
}

impl CandidateList {
    pub const MAX_SIZE: usize = 2; // TODO recount
}

// seeds: CANDIDATE, index (BE), period (BIG ENDIAN)
#[account]
pub struct CandidateAccount {
    // github user name, max 32 char
    author: String,
    // github repo name, max 32 char
    project: String,
    votes: u32,
    index: u16,
    period: u16
}

impl CandidateAccount {
    pub const MAX_SIZE: usize = 42; // TODO recount
}