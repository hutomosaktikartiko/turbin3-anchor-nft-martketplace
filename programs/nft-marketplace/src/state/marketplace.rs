use ::anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey,     // admin address
    pub fee: u16,          // fee in points (e.g., 250 = 2.5%)
    pub bump: u8,          // pda marketplace
    pub treasury_bump: u8, // pda treasury
    pub rewards_bump: u8,  // pda rewards
    pub name: String,      // marketplace name
}

impl Space for Marketplace {
    // 8: discrimator
    // 32: admin address
    // 2: fee
    // 1 x 3: all bump seed pda
    // 4: 'name' total characters
    // 32: value of 'name' max size
    const INIT_SPACE: usize = 8 + 32 + 2 + 1 + 1 + 1 + (4 + 32);
}
