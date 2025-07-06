#![allow(unexpected_cfgs)]
use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

declare_id!("7AYFMrmdmCfyX6YWv3ddrwKr6VM13vz7WNLjUyKWNE2R");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
    
    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        ctx.accounts.close_vault()
    }
    
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [b"state", signer.key().as_ref()],
        bump,
        space = 8 + VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>,

     #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info>{
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()>{
        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit <'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

impl<'info> Deposit<'info>{
    pub fn deposit(&mut self, amount:  u64) -> Result<()>{
        
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw <'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

impl<'info> Withdraw<'info>{
     pub fn withdraw(&mut self, amount:  u64) -> Result<()>{
        
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info(),
          
        };
        let seeds = [
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]

        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct CloseVault <'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
        close = signer
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

impl<'info> CloseVault<'info>{
     pub fn close_vault(&mut self) -> Result<()>{
        
        let cpi_program = self.system_program.to_account_info();


        let cpi_accounts = Transfer{
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info(),
          
        };
        let pda_signing_seeds = [
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]

        ];
        let seeds = [&pda_signing_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &seeds);

        transfer(cpi_ctx, self.vault.lamports())
    }
}

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 1+1;
}