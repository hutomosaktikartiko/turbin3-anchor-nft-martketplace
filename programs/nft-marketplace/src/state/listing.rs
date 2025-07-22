use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub maker: Pubkey,      // maker address
    pub maker_mint: Pubkey, // mint address
    pub price: u64,         // price in SOL
    pub bump: u8,           // pda listing
}

impl Space for Listing {
    // 8: discrimator
    // 32: maker address
    // 32: mint address
    // 8: price
    // 1: bump
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}
