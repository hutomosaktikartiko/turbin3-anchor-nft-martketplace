use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>, // seller must sign transaction

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>, // marketplace verification

    pub maker_mint: InterfaceAccount<'info, Mint>, // nft address

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>, // seller token account for NFT

    #[account(
        init, // create this account
        payer = maker, // maker pay this
        associated_token::mint = maker_mint,
        associated_token::authority = listing, // gave authority to pda listing 
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>, // token account for nft when listing

    #[account(
        init,
        payer = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
        space = Listing::INIT_SPACE,
    )]
    pub listing: Account<'info, Listing>, // account for listing details

    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(), // validate nft is part of collection or not
        constraint = metadata.collection.as_ref().unwrap().verified == true, // validate nft verify status
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>, // unique NFT

    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> List<'info> {
    pub fn create_listing(&mut self, price: u64, bumps: &ListBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            maker: self.maker.key(),
            maker_mint: self.maker_mint.key(),
            price,
            bump: bumps.listing,
        });

        Ok(())
    }

    pub fn deposit_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.maker_ata.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        // transfer 1 NFT and set decimals as 0, using maker_mint.decimals for safety
        transfer_checked(cpi_context, 1, self.maker_mint.decimals)?;

        Ok(())
    }
}
