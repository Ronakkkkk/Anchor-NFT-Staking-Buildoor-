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

declare_id!("9QtXBKnLPGkv6EmtF4Pk8p75NSH8wPLXkP1K3Ydshb5k");

#[program]
pub mod anchor_nft_staking {
    use super::*;

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        Ok(())
    }

    pub fn redeem(ctx: Context<Redeem>) -> Result<()> {
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        Ok(())
    }
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
    fn id() -. Pubkey{
        MetadataTokenId
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        associated_token::mint-nft_mint,
        associated_token::authority=user
    )]

    pub nft_token_account: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,
    ///CHECK: Manual Validation
    #[account(
        init_if_needed,
        payer=user,
        space= std::mem::size_of::<UserStakeInfo>() + 8,
        seeds= []
    )]

}

#[derive(Accounts)]
pub struct Initialize {}
