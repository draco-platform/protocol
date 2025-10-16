pub mod instructions;
pub mod state;
pub mod constants;

use anchor_lang::prelude::*;

pub use instructions::*;

declare_id!("Gudf3TTqxeBuUX8USrSzon9zVQ8s1UTcpZFHGVqEhZH1");

#[program]
pub mod draco_protocol {
    use super::*;
    pub fn initialize_authority(ctx: Context<InitializeAuthority>) -> Result<()> {
        instructions::initialize_authority::set_protocol_authority(ctx)?;
        Ok(())
    }

    pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
        msg!("draco_protocol::initialize_treasury");
        instructions::protocol_authority::enforce_protocol_authority(
            ctx.accounts.protocol_authority.authority,
            ctx.accounts.payer.key(),
        )?;

        instructions::initialize_treasury::contribute_treasury_account(&ctx)?;
        instructions::initialize_treasury::contribute_six_month_cliff_account(ctx)?;
        Ok(())
    }

    pub fn initialize_lottery_reward_factors(ctx: Context<InitializeLotteryRewardFactors>) -> Result<()> {
        msg!("draco_protocol::initialize_lottery_reward_factors");
        instructions::protocol_authority::enforce_protocol_authority(
            ctx.accounts.protocol_authority.authority,
            ctx.accounts.payer.key(),
        )?;
        instructions::reward_factors::initialize_lottery(ctx)?;
        Ok(())
    }   

    pub fn update_lottery_reward_factors(
        ctx: Context<UpdateLotteryRewardFactors>, 
        reward_full_match: f64, 
        reward_suit_match: f64, 
        reward_value_match: f64, 
        suit_streak_bonuses: [f64; 5], 
        value_streak_bonuses: [f64; 5], 
        jackpot_percentage: f64, 
        max_boost: f64, 
        curvature: f64,
        lock_divider: f64,
    ) -> Result<()> {
        msg!("draco_protocol::update_lottery_reward_factors");
        instructions::protocol_authority::enforce_protocol_authority(
            ctx.accounts.protocol_authority.authority,
            ctx.accounts.payer.key(),
        )?;
        instructions::reward_factors::update_lottery(ctx, 
            reward_full_match, 
            reward_suit_match, 
            reward_value_match, 
            suit_streak_bonuses, 
            value_streak_bonuses, 
            jackpot_percentage, max_boost, curvature, lock_divider)?;
        Ok(())
    }

    pub fn transfer_out_from_six_month_cliff(ctx: Context<TransferOutFromSixMonthCliff>) -> Result<()> {
        msg!("draco_protocol::transfer_out_from_six_month_cliff");
        instructions::protocol_authority::enforce_protocol_authority(
            ctx.accounts.protocol_authority.authority,
            ctx.accounts.payer.key(),
        )?;
        instructions::six_month_cliff::transfer_out(ctx)?;
        Ok(())
    }

    pub fn start_lottery(ctx: Context<StartLottery>, lottery_id: u64, lottery_name: String, lottery_description: String, lottery_type: u8, lottery_start_datetime: i64, lottery_end_datetime: i64, initial_prize_pool: u64, min_tokens_per_participant: u64) -> Result<()> {
        msg!("draco_protocol::start_lottery");
        instructions::protocol_authority::enforce_protocol_authority(
            ctx.accounts.protocol_authority.authority,
            ctx.accounts.payer.key(),
        )?;
        instructions::lottery::start(
            ctx, 
            lottery_id, 
            lottery_name, 
            lottery_description, 
            lottery_type, 
            lottery_start_datetime, 
            lottery_end_datetime, 
            initial_prize_pool, 
            min_tokens_per_participant
        )?;
        Ok(())
    }

    pub fn buy_lottery_ticket(ctx: Context<BuyLotteryTicket>, lottery_id: u64, combination: String, amount: u64) -> Result<()> {
        msg!("draco_protocol::buy_lottery_ticket");
        instructions::lottery::enforce_lottery_active(&ctx)?;
        let _ = instructions::lottery::verify_combination(&combination)?;
        instructions::lottery::verify_amount_on_type(&ctx, amount.clone())?;
        instructions::lottery::buy_ticket(ctx, lottery_id, amount, combination)?;
        Ok(())
    }

    pub fn commit_lottery_randomness(ctx: Context<CommitLotteryRandomness>, lottery_id: u64) -> Result<()> {
        msg!("draco_protocol::commit_lottery_randomness");
        instructions::protocol_authority::enforce_protocol_authority(
            ctx.accounts.protocol_authority.authority,
            ctx.accounts.payer.key(),
        )?;
        instructions::lottery::commit_randomness(ctx, lottery_id)?;
        Ok(())
    }

    pub fn reveal_lottery_randomness(ctx: Context<RevealLotteryRandomness>, lottery_id: u64) -> Result<()> {
        msg!("draco_protocol::reveal_lottery_randomness");
        instructions::protocol_authority::enforce_protocol_authority(
            ctx.accounts.protocol_authority.authority,
            ctx.accounts.payer.key(),
        )?;
        instructions::lottery::reveal_randomness(ctx, lottery_id)?;
        Ok(())
    }

    pub fn claim_lottery_prize_for_combination(ctx: Context<ClaimLotteryPrizeForCombination>, lottery_id: u64, combination: String) -> Result<()> {
        msg!("draco_protocol::claim_lottery_prize_for_combination");
        instructions::lottery::claim_prize_for_combination(ctx, lottery_id, combination)?;
        Ok(())
    }

    pub fn close_lottery(ctx: Context<CloseLottery>, lottery_id: u64) -> Result<()> {
        msg!("draco_protocol::close_lottery");
        instructions::protocol_authority::enforce_protocol_authority(
            ctx.accounts.protocol_authority.authority,
            ctx.accounts.payer.key(),
        )?;
        instructions::lottery::close(ctx, lottery_id)?;
        Ok(())
    }

    pub fn create_airdrop(ctx: Context<CreateAirdrop>, airdrop_id: u64, name: String, supply: u64, amount_per_claim: u64, start_datetime: i64, end_datetime: i64) -> Result<()> {
        msg!("draco_protocol::create_airdrop");
        instructions::protocol_authority::enforce_protocol_authority(
            ctx.accounts.protocol_authority.authority,
            ctx.accounts.payer.key(),
        )?;
        instructions::airdrop::create(ctx, airdrop_id, name, supply, amount_per_claim, start_datetime, end_datetime)?;
        Ok(())
    }

    pub fn claim_airdrop(ctx: Context<ClaimAirdrop>, airdrop_id: u64) -> Result<()> {
        msg!("draco_protocol::claim_airdrop");
        instructions::airdrop::claim(ctx, airdrop_id)?;
        Ok(())
    }

}

