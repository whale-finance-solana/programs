// 1. Import dependencies
use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::instruction::create_metadata_accounts_v3;

// 2. Declare Program ID (SolPG will automatically update this when you deploy)
declare_id!("55tC9joryrqBuUJjURE5i2pLLbzoFfx1K1hRWnMfigtF");

// 3. Define the program and instructions
#[program]
mod vault_minter {
    use super::*;
    pub fn init_vault(ctx: Context<InitToken>, metadata: InitVaultParams) -> Result<()> {
        let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("mint").unwrap()]];
        let signer = [&seeds[..]];

        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        
        // Create the token mint that represent the quota token
        invoke_signed(
            &create_metadata_accounts_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.mint.key(),
                metadata.name,
                metadata.symbol,
                metadata.uri,
                None,
                0,
                true,
                true,
                None,
                None,
                None,
            ),
            account_info.as_slice(),
            &signer,
        )?;

        msg!("Quota Token mint created successfully.");

        Ok(())
    }

    pub fn invest_vault(ctx: Context<InvestVault>, amount: u64) -> Result<()> {
        let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("mint").unwrap()]];
        let signer = [&seeds[..]];

        // Transfer the amount (in lamports) to the vault account

        // let transfer_lamport_instruction = anchor_lang::system_program::transfer(
        //     ctx.accounts.investor.to_account_info().key,
        //     ctx.accounts.vault.to_account_info().key,
        //     amount,
        // );

        // anchor_lang::solana_program::program::invoke_signed(
        //     &transfer_lamport_instruction,
        //     &[
        //         ctx.accounts.investor.to_account_info(),
        //         ctx.accounts.vault.clone(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        //     &[],
        // )?;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &signer,
            ),
            amount,
        )?;

        Ok(())
    }
}

// 4. Define the context for each instruction
#[derive(Accounts)]
#[instruction(params: InitVaultParams)]
pub struct InitToken<'info> {

    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"mint"],
        bump,
        payer = payer,
        mint::decimals = params.decimals,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK: account constraint checked in account trait
    #[account(address = mpl_token_metadata::id())]
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct InvestVault<'info> {


    #[account(mut)]
    pub investor: Signer<'info>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"mint"],
        bump,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// 5. Define the init token params
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitVaultParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}