use anchor_lang::prelude::*;
use crate::{errors::EscrowError, constants::{ANCHOR_DISCRIMINATOR_BYTES, PUBKEY_BYTES, U64_BYTES, U8_BYTES}};

#[account]
pub struct Escrow {
    pub maker: Pubkey,
    pub maker_token: Pubkey,
    pub taker_token: Pubkey,
    pub offer_amount: u64,
    pub seed: u64,
    pub expiry: u64,
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub escrow_bump: u8
}

impl Escrow {
    pub const LEN: usize = ANCHOR_DISCRIMINATOR_BYTES + 3 * PUBKEY_BYTES + 3 * U64_BYTES + 3 * U8_BYTES;

    pub fn check_expiry(&self) -> Result<()> {
        require!(self.expiry > Clock::get()?.slot, EscrowError::Expired);
        Ok(())
    }

    pub fn set_expiry_relative(&mut self, expiry: u64) -> Result<()> {
        require!(expiry.lt(&100_000), EscrowError::MaxExpiryExceeded);
        self.set_expiry_absolute( match expiry > 0 {
            true => Clock::get()?.slot + expiry,
            false => 0
        });
        Ok(())
    }

    pub fn set_expiry_absolute(&mut self, expiry: u64) {
        self.expiry = expiry;
    }
}
// TODO: add bumps
#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u16,
    pub bump: u8,
    pub treasury_bump: u8,
    pub name: String,
}


impl Marketplace {
    pub const LEN: usize = 8 + 32 + 2 + 1 + 1 + 4 + 32;
}

#[account]
pub struct Whitelist {
    pub bump: u8
}

impl Whitelist {
    pub const LEN: usize = 8 + 1;
}

// added expiry, highest_bid, Highest_bidder, reserve_price
#[account]
pub struct Listing {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub reserve_price: u64,
    pub bump: u8,
    pub auth_bump: u8,
    pub expiry: i64,
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
}

#[account]
pub struct Refund {
    
}

impl Listing {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1 + 1;
}
