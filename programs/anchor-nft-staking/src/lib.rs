use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Approve, Mint, MintTo, Revoke, Token, TokenAccount},
};
use mpl_token_metadata::{
    instruction::{freeze_delegated_account, thaw_delegated_account},
    ID as MetadataTokenId,
};

declare_id!("FpajM5sdtsdoonHRA462iDg9zYZzzXZ6Wa8MPiYY4L4p");

#[program]
pub mod anchor_nft_staking {
    use super::*;

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        let clock = Clock::get().unwrap();
        msg!("Approving delegate");

        let cpi_approve_program = ctx.accounts.token_program.to_account_info();
        let cpi_approve_accounts = Approve {
            to: ctx.accounts.nft_token_account.to_account_info(),
            delegate: ctx.accounts.program_authority.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_approve_ctx = CpiContext::new(cpi_approve_program, cpi_approve_accounts);
            token::approve(cpi_approve_ctx,1)?;


        Ok(())


}


#[account]
pub struct UserStakeInfo{
    pub token_account: Pubkey,
    pub stake_start_time: i64,
    pub last_stake_redeem: i64,
    pub user_pubkey: Pubkey,
    pub stake_state: StakeState,
    pub is_initialized: bool
}

#[derive(Debug, PartialEq, AnchorDeserialize, AnchorSerialize, Clone)]
pub enum StakeState{
    Unstaked,
    Staked,
}

impl Default for StakeState{
    fn default() -> Self {
        StakeState::Unstaked
    }
}

#[derive(Clone)]
pub struct Metadata;

impl anchor_lang::Id for Metadata{
    fn id() -> Pubkey{
        MetadataTokenId
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        associated_token::mint =nft_mint,
        associated_token::authority= user,
    )]
    pub nft_token_account: Account<'info, TokenAccount>,

    pub nft_mint: Account<'info, Mint>,

    ///CHECK: Manual Validation 
    #[account(owner=MetadataTokenId)]
    pub nft_edition: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer=user,
        space= std::mem::size_of::<UserStakeInfo>() + 8,
        seeds= [user.key().as_ref(), nft_token_account.key().as_ref()],
        bump,
    )]
    pub stake_state: Account<'info, UserStakeInfo>,

    ///CHECK: Manual Validation
    #[account(mut, seeds= ["authority".as_bytes().as_ref()], bump)]
    pub program_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,

}


