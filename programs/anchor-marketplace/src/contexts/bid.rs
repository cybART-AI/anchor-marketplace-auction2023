use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer as SplTransfer, transfer as spl_transfer, CloseAccount, close_account}, associated_token::AssociatedToken};
use crate::{state::Marketplace, state::Whitelist, state::Listing, refund::Refund};

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(mut)]
    taker: Signer<'info>,
    #[account(mut)]
    /// CHECK: This is safe
    maker: UncheckedAccount<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::authority = taker,
        associated_token::mint = maker_mint
    )]
    taker_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"auth", maker_mint.key().as_ref()],
        bump = listing.auth_bump,
        token::authority = vault,
        token::mint = maker_mint
    )]
    vault: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump
    )]
    
    treasury: SystemAccount<'info>,
    maker_mint: Account<'info, Mint>,
    collection_mint: Account<'info, Mint>,
    #[account(
        seeds = [marketplace.key().as_ref(), collection_mint.key().as_ref()],
        bump = whitelist.bump
    )]
    whitelist: Account<'info, Whitelist>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        seeds = [whitelist.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump
    )]
    listing: Account<'info, Listing>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

impl<'info> Bid<'info> {
    // add check to make sure it's greater than highest bid and time isn't expired
    // add function to add 10 minutes to the auction after a new highest bid
    pub fn send_sol(&self) -> Result<()> {
        let accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info()
        };
        let ctx = CpiContext::new(
            self.system_program.to_account_info(), 
            accounts
        );
        transfer(ctx, self.listing.price)
    }

    pub fn send_nft(&self) -> Result<()> {
        let accounts = SplTransfer {
            from: self.vault.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.vault.to_account_info()
        };

        let seeds = [b"auth", &self.maker_mint.key().to_bytes()[..], &[self.listing.auth_bump]];
        let signer_seeds = &[&seeds[..]][..];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        spl_transfer(ctx, 1)
    }

    pub fn close_mint_ata(&mut self) -> Result<()> {
        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.vault.to_account_info()
        };

        let seeds = [b"auth", &self.maker_mint.key().to_bytes()[..], &[self.listing.auth_bump]];
        let signer_seeds = &[&seeds[..]][..];
        
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        close_account(ctx)
    }

}
