#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod context;
pub mod state;

pub use context::*;
pub use state::*;

declare_id!("ALakMxNbgg1W9mwC9xTQi4ASDCzGZis8AHZPYRmCi1Cw");

#[program]
pub mod nft_marketplace {
    use super::*;

    // create new NFT marketplace
    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;

        Ok(())
    }

    // listinf NFT
    pub fn listing(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;

        Ok(())
    }

    // cancel NFT listing
    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist()?; // cancel NFT listing
        ctx.accounts.close_mint_vault()?; // close account

        Ok(())
    }

    // buy NFT
    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol()?; // pay with SOL
        ctx.accounts.receive_nft()?; // transfer NFT
        ctx.accounts.receive_rewards()?; // buyer get rewards
        ctx.accounts.close_mint_vault()?; // close account

        Ok(())
    }
}
