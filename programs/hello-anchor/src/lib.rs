use anchor_lang::prelude::*;
use anchor_spl::{token::{Transfer, transfer, Token, TokenAccount}, associated_token::AssociatedToken};
use std::collections::BTreeMap;
use solana_program::entrypoint::ProgramResult;

declare_id!("F9ue8AWzZYrSKyE7PRnh8tR1Hgzxf8LSYrBZY64FjXtT");

pub fn calculate_reward(amount: u64) -> u64 {
    amount / 10 // 10% reward
}

#[program]
mod hello_anchor {
    use super::*;
    
    pub fn initialize_prediction_market(ctx: Context<InitializePredictionMarket>) -> ProgramResult {
        let state = &mut ctx.accounts.state_data;
        state.bets = Vec::new();
        Ok(())
    }

    pub fn place_bet(
        ctx: Context<BetParams>, 
        prediction_name: String, 
        prediction: PredictionType, 
        amount: u64, 
        start_time: i64, 
        expiration_time: i64
    ) -> ProgramResult {
        let user_token_account = &ctx.accounts.to_token_account;
        if user_token_account.amount < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        let current_time = Clock::get()?.unix_timestamp;
        if expiration_time <= current_time || start_time < current_time {
            return Err(ProgramError::InvalidAccountData);
        }

        let new_bet = Bet {
            user: *ctx.accounts.signer.key,
            prediction_name,
            prediction,
            amount,
            start_time,
            expiration_time,
            is_open: true,
        };

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        transfer(cpi_context, amount)?;

        let state = &mut ctx.accounts.state_data;
        state.bets.push(new_bet);

        Ok(())
    }

    pub fn resolve_bet(ctx: Context<BetParams>, bet_index: u64) -> ProgramResult {
        let prediction_market = &mut ctx.accounts.state_data;
        let current_time = Clock::get()?.unix_timestamp;
        let bet = &mut prediction_market.bets[bet_index as usize];
        if current_time < bet.expiration_time {
            return Err(ProgramError::InvalidAccountData);
        }

        bet.is_open = false;

        let bump = &[ctx.bumps.vault];
        let seeds: &[&[u8]] = &[b"vault".as_ref(), bump];
        let signer_seeds = &[&seeds[..]];
        let reward_amount = calculate_reward(bet.amount);

            let cpi_accounts = Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.to_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            };
            let cpi_context = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
            ).with_signer(signer_seeds);
            transfer(cpi_context, bet.amount + reward_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePredictionMarket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer = signer, space = 8 + 8, seeds = [b"predictionmarket"], bump)]
    pub state_data: Account<'info, PredictionMarket>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BetParams<'info> {
    #[account(mut, seeds = [b"predictionmarket"], bump)]
    pub state_data: Account<'info, PredictionMarket>,
    #[account(
        mut,
        seeds = [b"vault".as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(mut,  seeds = [b"tokenaccount"], bump)]
    pub to_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct PredictionMarket {
    pub bets: Vec<Bet>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Bet {
    pub user: Pubkey,
    pub prediction_name: String,
    pub prediction: PredictionType,
    pub amount: u64,
    pub start_time: i64,
    pub expiration_time: i64,
    pub is_open: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PredictionType {
    Higher,
    Lower,
}
