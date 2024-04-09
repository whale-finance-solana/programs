use anchor_lang::{prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ mint_to, initialize_mint, InitializeMint, MintTo, Token},
};
use solana_program::pubkey;


declare_id!("55tC9joryrqBuUJjURE5i2pLLbzoFfx1K1hRWnMfigtF");

const TREASURY_PUBKEY: Pubkey = pubkey!("BLSzuy78aPbCw5ug7bNiGeq29cbuJa1LfGaL7C1dkzj8");

#[program]
mod vault_minter {
    use super::*;


    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {

        //Staking SOL to create vault
        //  let cpi_context = CpiContext::new(
        //      ctx.accounts.system_program.to_account_info(), 
        //      anchor_lang::system_program::Transfer {
        //          from: ctx.accounts.signer.to_account_info(),
        //          to: ctx.accounts.treasury_account.to_account_info(),
        //  });
        //  anchor_lang::system_program::transfer(cpi_context, amount_staking)?;

        //creating vault with mint token as quota token
        anchor_lang::system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(), 
                anchor_lang::system_program::CreateAccount { 
                    from: ctx.accounts.signer.to_account_info(), 
                    to: ctx.accounts.mint_token.to_account_info() }
            ), 
            10_000_000, 
            82, 
            ctx.accounts.token_program.key
        )?;

        
        //initializing mint token
        initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint{mint:ctx.accounts.mint_token.to_account_info(),rent:ctx.accounts.rent.to_account_info()}
            ), 
            6, 
            ctx.accounts.signer.key, 
            Some(ctx.accounts.signer.key)
        )?;
      
      
        anchor_spl::associated_token::create(
            CpiContext::new(
                ctx.accounts.associate_token_program.to_account_info(), 
                anchor_spl::associated_token::Create { 
                    payer: ctx.accounts.signer.to_account_info(), 
                    associated_token: ctx.accounts.token_account.to_account_info(), 
                    authority: ctx.accounts.signer.to_account_info(), 
                    mint: ctx.accounts.mint_token.to_account_info(), 
                    system_program: ctx.accounts.system_program.to_account_info(), 
                    token_program: ctx.accounts.token_program.to_account_info() 
                }
            )
        )?;
      
        // mint_to(
        //     CpiContext::new(
        //         ctx.accounts.token_account.to_account_info(), 
        //         MintTo{
        //             authority:ctx.accounts.signer.to_account_info(),
        //             int:ctx.accounts.mint_token.to_account_info(),
        //             to:ctx.accounts.token_account.to_account_info()
        //         }
        //     ), 
        //     quantity
        // )?;
      
        Ok(())
      }

    pub fn invest_vault(ctx: Context<InvestVault>, amount: u64) -> Result<()> {
        // let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("mint").unwrap()]];
        // let signer = [&seeds[..]];
            
        // Transfer the amount (in lamports) to the vault account
            
        // let transfer_lamport_instruction = anchor_lang::system_program::transfer(
        //     &ctx.accounts.signer.to_account_info().key,
        //     &ctx.accounts.mint_token.to_account_info().key,
        //     amount,
        // );

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.mint_token.to_account_info(),
        });
        anchor_lang::system_program::transfer(cpi_context, amount)?;

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
            CpiContext::new(
                ctx.accounts.token_account.to_account_info(), 
                MintTo{
                    authority:ctx.accounts.signer.to_account_info(),
                    mint:ctx.accounts.mint_token.to_account_info(),
                    to:ctx.accounts.token_account.to_account_info()
                }
            ), 
            amount
        )?;
        
    Ok(())
}


}


#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub mint_token:Signer<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
    ///CHECK:
    #[account(mut)]
    pub token_account:AccountInfo<'info>,

    // #[account(mut, 
    //     address = BLSzuy78aPbCw5ug7bNiGeq29cbuJa1LfGaL7C1dkzj8)]
    // pub treasury_account:AccountInfo<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
    pub rent:Sysvar<'info,Rent>
}

#[derive(Accounts)]
pub struct InvestVault<'info> {
    #[account(mut)]
    pub mint_token:Signer<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
    ///CHECK:
    #[account(mut)]
    pub token_account:AccountInfo<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associate_token_program:Program<'info,AssociatedToken>,
    pub rent:Sysvar<'info,Rent>

}
