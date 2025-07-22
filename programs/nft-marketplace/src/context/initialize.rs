use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)] // marketplace name from user input
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = Marketplace::INIT_SPACE,
        seeds = [b"marketplace".as_ref(), name.as_str().as_bytes()],
        bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 9,
        mint::authority = marketplace, // gave authority to pda marketplace
        seeds = [b"rewards".as_ref(), marketplace.key().as_ref()],
        bump,
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>, // create new token

    #[account(
        seeds = [b"treasury".as_ref(), marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>, // vault pda for transaction fee in SOL

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_bump: bumps.reward_mint,
            name: name,
        });

        Ok(())
    }
}
