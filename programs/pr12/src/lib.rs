use anchor_lang::prelude::*;

declare_id!("3JqCcV9taaDixgaVySQkcPiFWzzchoH5YL5PCoAnYC4e");  // Замените "YourProgramPubkeyHere" на реальный публичный ключ программы.

#[program]
pub mod my_solana_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, max_tokens: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.owner = *ctx.accounts.owner.key;
        state.max_tokens = max_tokens;
        state.current_tokens = 0;
        state.total_minted_tokens = 0;  // Инициализация нового поля для общего количества выпущенных токенов
        Ok(())
    }

    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;

        // Проверка владельца
        if *ctx.accounts.owner.key != state.owner {
            return Err(ErrorCode::Unauthorized.into());
        }

        // Проверка количества токенов
        if state.current_tokens + amount > state.max_tokens {
            return Err(ErrorCode::TokenLimitExceeded.into());
        }

        // Обработка логики минтинга
        state.current_tokens += amount;
        state.total_minted_tokens += amount;  // Обновление общего количества выпущенных токенов
        
        // Аудит логов
        msg!("Minted {} tokens. Total: {}", amount, state.current_tokens);

        Ok(())
    }

    pub fn freeze_account(ctx: Context<FreezeAccount>) -> Result<()> {
        let account = &mut ctx.accounts.account;
        account.is_frozen = true;
        Ok(())
    }

    pub fn unfreeze_account(ctx: Context<UnfreezeAccount>) -> Result<()> {
        let account = &mut ctx.accounts.account;
        account.is_frozen = false;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>, amount: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;

        // Проверка на возврат
        if amount > state.current_tokens {
            return Err(ErrorCode::InsufficientTokens.into());
        }

        state.current_tokens -= amount;

        // Логика возврата токенов
        msg!("Refunded {} tokens. Remaining: {}", amount, state.current_tokens);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8 + 8)]  // Пример: 8 (дата аккаунта) + 32 (Pubkey) + 8 (max_tokens) + 8 (current_tokens) + 8 (total_minted_tokens)
    pub state: Account<'info, State>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct FreezeAccount<'info> {
    #[account(mut)]
    pub account: Account<'info, UserAccount>,
}

#[derive(Accounts)]
pub struct UnfreezeAccount<'info> {
    #[account(mut)]
    pub account: Account<'info, UserAccount>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
}

#[account]
pub struct State {
    pub owner: Pubkey,
    pub max_tokens: u64,
    pub current_tokens: u64,
    pub total_minted_tokens: u64,  // Добавлено для отслеживания общего количества выпущенных токенов
}

#[account]
pub struct UserAccount {
    pub is_frozen: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access.")]
    Unauthorized,
    #[msg("Token limit exceeded.")]
    TokenLimitExceeded,
    #[msg("Insufficient tokens for refund.")]
    InsufficientTokens,
}
