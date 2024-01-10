use anchor_lang::prelude::*;

declare_id!("9QtXBKnLPGkv6EmtF4Pk8p75NSH8wPLXkP1K3Ydshb5k");

#[program]
pub mod anchor_nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
