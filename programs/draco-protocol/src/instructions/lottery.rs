use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token_interface::{ Mint, TokenInterface, TokenAccount}};
use switchboard_on_demand::RandomnessAccountData;

use crate::{instructions, state::protocol_authority::ProtocolAuthority};
use crate::state::lottery::LotteryState;
use crate::state::lottery_reward_factors::LotteryRewardFactors;
use crate::state::ticket::Ticket;
use crate::state::protocol_error::ErrorCode;

use crate::instructions::pda_owned_token_accounts;

use crate::constants::{
    VALID_SUITS, VALID_VALUES, COMBINATION_LENGTH, PAY_LOTTERY_TYPE, LOCK_LOTTERY_TYPE,
    LOTTERY_CLOSE_TIME_BUFFER,
};

#[derive(Accounts)]
#[instruction(lottery_id: u64)]
pub struct StartLottery<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"treasury".as_ref()],
        bump
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        seeds = [
            b"lottery_token_account".as_ref(), 
            lottery_id.to_le_bytes().as_ref()
        ],
        bump,
        token::mint = token_mint,
        token::authority = lottery_token_account,
    )]
    pub lottery_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        seeds = [
            b"lottery_state".as_ref(), 
            lottery_id.to_le_bytes().as_ref()
        ],
        bump,
        space = 8 + LotteryState::INIT_SPACE,
    )]
    pub lottery_state: Account<'info, LotteryState>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"protocol_authority".as_ref()],
        bump
    )]
    pub protocol_authority: Account<'info, ProtocolAuthority>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(lottery_id: u64, combination: String)]
pub struct BuyLotteryTicket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub payer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            b"lottery_token_account".as_ref(), 
            lottery_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub lottery_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [
            b"ticket".as_ref(), 
            lottery_id.to_le_bytes().as_ref(), 
            payer.key().to_bytes().as_ref(),
            combination.as_ref()
            ],
        bump,
        space = 8 + Ticket::INIT_SPACE,
    )]
    pub ticket: Account<'info, Ticket>,

    #[account(
        mut,
        seeds = [
            b"lottery_state".as_ref(), 
            lottery_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub lottery_state: Account<'info, LotteryState>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(lottery_id: u64)]
pub struct CommitLotteryRandomness<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"lottery_state".as_ref(), 
            lottery_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub lottery_state: Account<'info, LotteryState>,

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"protocol_authority".as_ref()],
        bump
    )]
    pub protocol_authority: Account<'info, ProtocolAuthority>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(lottery_id: u64)]
pub struct RevealLotteryRandomness<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"lottery_state".as_ref(), 
            lottery_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub lottery_state: Account<'info, LotteryState>,

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"protocol_authority".as_ref()],
        bump
    )]
    pub protocol_authority: Account<'info, ProtocolAuthority>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(lottery_id: u64, combination: String)]
pub struct ClaimLotteryPrizeForCombination<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub payer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds =[
            b"lottery_token_account".as_ref(), 
            lottery_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub lottery_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"treasury".as_ref()],
        bump
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            b"ticket".as_ref(), 
            lottery_id.to_le_bytes().as_ref(), 
            payer.key().to_bytes().as_ref(),
            combination.as_ref()
            ],
        bump,
    )]
    pub ticket: Account<'info, Ticket>,

    #[account(
        mut,
        seeds = [
            b"lottery_state".as_ref(), 
            lottery_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub lottery_state: Account<'info, LotteryState>,

    #[account(
        mut,
        seeds = [b"lottery_reward_factors".as_ref()],
        bump
    )]
    pub lottery_reward_factors: Account<'info, LotteryRewardFactors>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(lottery_id: u64)]
pub struct CloseLottery<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"lottery_state".as_ref(), 
            lottery_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub lottery_state: Account<'info, LotteryState>,

    #[account(
        mut,
        seeds =[
            b"lottery_token_account".as_ref(), 
            lottery_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub lottery_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"treasury".as_ref()],
        bump
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"protocol_authority".as_ref()],
        bump
    )]
    pub protocol_authority: Account<'info, ProtocolAuthority>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn start(
    ctx: Context<StartLottery>, 
    lottery_id: u64, 
    lottery_name: String, 
    lottery_description: String, 
    lottery_type: u8, 
    lottery_start_datetime: i64, 
    lottery_end_datetime: i64, 
    initial_prize_pool: u64, 
    min_tokens_per_participant: u64
) -> Result<()> {
    msg!("Starting lottery");

    require!(
        lottery_type == PAY_LOTTERY_TYPE || lottery_type == LOCK_LOTTERY_TYPE, 
        ErrorCode::InvalidLotteryType
    );
    require!(
        lottery_start_datetime < lottery_end_datetime, 
        ErrorCode::InvalidLotteryStartEndDatetime
    );
    require!(
        initial_prize_pool > 0, 
        ErrorCode::InvalidInitialPrizePool
    );
    require!(
        min_tokens_per_participant > 0, 
        ErrorCode::InvalidMinTokensPerParticipant
    );

    let lottery_state = &mut ctx.accounts.lottery_state;
    lottery_state.lottery_id = lottery_id;
    lottery_state.lottery_name = lottery_name;
    lottery_state.lottery_description = lottery_description;
    lottery_state.lottery_type = lottery_type;
    lottery_state.lottery_start_datetime = lottery_start_datetime;
    lottery_state.lottery_end_datetime = lottery_end_datetime;
    lottery_state.initial_prize_pool = initial_prize_pool;
    lottery_state.accumulated_prize_pool = initial_prize_pool;
    lottery_state.min_tokens_per_participant = min_tokens_per_participant;
    lottery_state.winning_combination = None;
    lottery_state.participants_count = 0;
    lottery_state.randomness_account = Pubkey::default();
    lottery_state.is_closed = false;

    let scaled_amount = instructions::utils::get_scaled_amount(
        initial_prize_pool,
        ctx.accounts.token_mint.decimals
    )?;

    let _ = pda_owned_token_accounts::withdraw(
        scaled_amount,
        &[b"treasury"],
        ctx.bumps.treasury_token_account,
        &ctx.accounts.treasury_token_account,
        &ctx.accounts.lottery_token_account,
        &ctx.accounts.token_mint,
        &ctx.accounts.token_program,
    );

    msg!("Lottery with lottery_id {} started successfully", lottery_id);
    Ok(())
}

pub fn enforce_lottery_active(ctx: &Context<BuyLotteryTicket>) -> Result<()> {
    msg!("Enforcing lottery active");
    let lottery_state = &ctx.accounts.lottery_state;
    msg!("Lottery start date {}", lottery_state.lottery_start_datetime);
    msg!("Lottery end date {}", lottery_state.lottery_end_datetime);
    msg!("Current time {}",Clock::get()?.unix_timestamp);
    require!(
        lottery_state.lottery_end_datetime > Clock::get()?.unix_timestamp, 
        ErrorCode::LotteryFinished
    );
    require!(
        lottery_state.lottery_start_datetime < Clock::get()?.unix_timestamp, 
        ErrorCode::LotteryNotStarted
    );
    require!(
        !lottery_state.is_closed, 
        ErrorCode::LotteryClosed
    );
    Ok(())
}

pub fn verify_combination(combination: &String) -> Result<Vec<(char, char)>> {
    msg!("Verifying combination {}", combination);
    
    require!(combination.len() == COMBINATION_LENGTH, ErrorCode::InvalidCombinationLength);
    
    let chars: Vec<char> = combination.chars().collect();
    let mut cards = Vec::new();
    
    for i in (0..COMBINATION_LENGTH).step_by(2) {
        let suit = chars[i];
        let value = chars[i + 1];
        
        require!(VALID_SUITS.contains(&suit), ErrorCode::InvalidCombinationSuit);
        
        require!(VALID_VALUES.contains(&value), ErrorCode::InvalidCombinationValue);

        cards.push((suit, value));
    }
    
    Ok(cards)
}

pub fn verify_amount_on_type(ctx: &Context<BuyLotteryTicket>, amount: u64) -> Result<()> {
    msg!("Verifying amount on type");
    let lottery_type = ctx.accounts.lottery_state.lottery_type;
    let min_tokens_per_participant = ctx.accounts.lottery_state.min_tokens_per_participant;
    if lottery_type == PAY_LOTTERY_TYPE {
        require!(
               amount >= min_tokens_per_participant 
            && amount % min_tokens_per_participant == 0, 
            ErrorCode::InvalidAmount
        );
    } else if lottery_type == LOCK_LOTTERY_TYPE {
        require!(amount >= min_tokens_per_participant, ErrorCode::InvalidAmount);
    }
    Ok(())
}

pub fn buy_ticket(
    ctx: Context<BuyLotteryTicket>, 
    lottery_id: u64, 
    amount: u64, 
    combination: String
) -> Result<()> {
    msg!("Buying ticket");
    let ticket = &mut ctx.accounts.ticket;
    if !ticket.is_initialized {
        ticket.lottery_id = lottery_id;
        ticket.combination = combination;
        ticket.amount = amount;
        ticket.participant = ctx.accounts.payer.key();
        ticket.is_claimed = false;
        ticket.is_initialized = true;
    } else {
        ticket.amount += amount;
    }

    let lottery_state = &mut ctx.accounts.lottery_state;
    lottery_state.participants_count += 1;
    lottery_state.accumulated_prize_pool += amount;

    let scaled_amount = instructions::utils::get_scaled_amount(
        amount,
        ctx.accounts.token_mint.decimals
    )?;

    let _ = pda_owned_token_accounts::contribute(
        scaled_amount, 
        &ctx.accounts.payer_token_account, 
        &ctx.accounts.lottery_token_account, 
        &ctx.accounts.payer, 
        &ctx.accounts.token_mint, 
        &ctx.accounts.token_program
    );
    msg!(
        "Bought ticket from lottery_id {} with amount {} and combination {} successfully", 
        ticket.lottery_id, ticket.amount, ticket.combination
    );
    Ok(())
}

pub fn commit_randomness(ctx: Context<CommitLotteryRandomness>, _lottery_id: u64) -> Result<()> {
    msg!("Committing randomness");

    let clock = Clock::get()?;
    let lottery_state = &mut ctx.accounts.lottery_state;
    let randomness_data = RandomnessAccountData::parse(
        ctx.accounts.randomness_account.data.borrow(),
    ).unwrap();

    require!(
        lottery_state.lottery_end_datetime < clock.unix_timestamp, 
        ErrorCode::CantCommitOnNotFinishedLottery
    );
    require!(
        !lottery_state.is_closed, 
        ErrorCode::LotteryClosed
    );
    require!(
        lottery_state.winning_combination.is_none(), 
        ErrorCode::CombinationAlreadySet
    );
    require!(
        randomness_data.seed_slot == clock.slot - 1,
        ErrorCode::RandomnessAlreadyRevealed
    );

    lottery_state.randomness_account = ctx.accounts.randomness_account.key();
    Ok(())
}

pub fn generate_combination_from_randomness(randomness_data: &[u8]) -> String {
    let mut available_cards: Vec<usize> = (0..52).collect();
    let mut combination = String::new();
    let mut byte_index = 0;
    
    for _ in 0..4 {
        // Ensure we have enough bytes
        if byte_index + 1 >= randomness_data.len() {
            byte_index = 0; // Wrap around if needed
        }
        
        // Use 2 bytes for better distribution
        let random_value = ((randomness_data[byte_index] as u16) << 8) 
                         | (randomness_data[byte_index + 1] as u16);
        byte_index += 2;
        
        let cards_remaining = available_cards.len();
        let selected_index = (random_value as usize) % cards_remaining;
        let card_index = available_cards.remove(selected_index);
        
        let suit_index = card_index / 13;
        let value_index = card_index % 13;
        
        combination.push(VALID_SUITS[suit_index]);
        combination.push(VALID_VALUES[value_index]);
    }
    
    combination
}

pub fn reveal_randomness(ctx: Context<RevealLotteryRandomness>, _lottery_id: u64) -> Result<()> {
    msg!("Revealing randomness");

    let clock = Clock::get()?;
    let lottery_state = &mut ctx.accounts.lottery_state;

    require!(
        lottery_state.randomness_account == ctx.accounts.randomness_account.key(), 
        ErrorCode::IncorrectRandomnessAccount
    );
    require!(
        lottery_state.lottery_end_datetime < clock.unix_timestamp, 
        ErrorCode::CantRevealOnNotFinishedLottery
    );
    require!(
        lottery_state.winning_combination.is_none(), 
        ErrorCode::CombinationAlreadySet
    );
    require!(!lottery_state.is_closed, ErrorCode::LotteryClosed);

    let randomness_data = 
        RandomnessAccountData::parse(ctx.accounts.randomness_account.data.borrow()).unwrap();
    let revealed_random_value = randomness_data.get_value(clock.slot)
        .map_err(|_| ErrorCode::RandomnessNotResolved)?;

    msg!("Randomness result: {:?}", revealed_random_value);

    let combination = generate_combination_from_randomness(
        &revealed_random_value
    );
    lottery_state.winning_combination = Some(combination);

    Ok(())
}

pub fn calculate_growth_factor(intital_value: u64, final_value: u64, max_boost: f64, curvature: f64) -> f64 {
    if final_value <= intital_value || intital_value == 0 {
        return 1.0;
    }
    
    let r = final_value as f64 / intital_value as f64;
    1.0 + max_boost * (1.0 - r.powf(-curvature))
}

pub fn calculate_num_tickets_reward(amount_payed: u64, ticket_price: u64, lottery_type: u8, max_boost: f64, curvature: f64) -> u64 {
    if lottery_type == PAY_LOTTERY_TYPE {
        return amount_payed / ticket_price;
    } else if lottery_type == LOCK_LOTTERY_TYPE {
        return calculate_growth_factor(
            ticket_price, 
            amount_payed,
            max_boost,
            curvature
        ).round() as u64;
    }
    return 1;
}

pub fn calculate_prize(
    winning_combination: &String, 
    combination: &String, 
    lottery_type: u8, 
    amount_payed: u64, 
    initial_prize_pool: u64, 
    accumulated_prize_pool: u64, 
    ticket_price: u64,
    reward_factors: &Account<'_, LotteryRewardFactors>
) -> Result<u64> {
    msg!("Calculating prize for combination {} given winning combination {} with lottery type {}", combination, winning_combination, lottery_type);

    let mut reward = 0.0;

    let computed_ticket_price: f64 = if lottery_type == PAY_LOTTERY_TYPE {
        ticket_price as f64
    } else {
        ticket_price as f64/reward_factors.lock_divider
    };

    let winning_cards = verify_combination(&winning_combination)?;
    let ticket_cards = verify_combination(&combination)?;
    
    let growth_factor = calculate_growth_factor(
        initial_prize_pool, 
        accumulated_prize_pool,
        reward_factors.max_boost,
        reward_factors.curvature
    );

    let mut suit_streak = 1usize;
    let mut value_streak = 1usize;
    
    for i in 0..4 {
        let (ticket_suit, ticket_value) = ticket_cards[i];
        let (winning_suit, winning_value) = winning_cards[i];
        
        if ticket_suit == winning_suit && ticket_value == winning_value {
            reward += reward_factors.reward_full_match * computed_ticket_price * growth_factor;
        } else if ticket_suit == winning_suit {
            reward += reward_factors.reward_suit_match * computed_ticket_price * growth_factor;
        } else if ticket_value == winning_value {
            reward += reward_factors.reward_value_match * computed_ticket_price * growth_factor;
        }
        
        if i > 0 {
            let (prev_ticket_suit, _) = ticket_cards[i - 1];
            let (prev_winning_suit, _) = winning_cards[i - 1];
            
            if prev_ticket_suit == prev_winning_suit && ticket_suit == winning_suit {
                suit_streak += 1;
            } else {
                if suit_streak > 1 && suit_streak < reward_factors.suit_streak_bonuses.len() {
                    reward += reward_factors.suit_streak_bonuses[suit_streak] * computed_ticket_price * growth_factor;
                }
                suit_streak = if ticket_suit == winning_suit { 1 } else { 0 };
            }
        } else {
            suit_streak = if ticket_suit == winning_suit { 1 } else { 0 };
        }
        
        if i > 0 {
            let (_, prev_ticket_value) = ticket_cards[i - 1];
            let (_, prev_winning_value) = winning_cards[i - 1];
            
            if prev_ticket_value == prev_winning_value && ticket_value == winning_value {
                value_streak += 1;
            } else {
                if value_streak > 1 && value_streak < reward_factors.value_streak_bonuses.len() {
                    reward += reward_factors.value_streak_bonuses[value_streak] * computed_ticket_price * growth_factor;
                }
                value_streak = if ticket_value == winning_value { 1 } else { 0 };
            }
        } else {
            value_streak = if ticket_value == winning_value { 1 } else { 0 };
        }
    }
    
    if suit_streak > 1 && suit_streak < reward_factors.suit_streak_bonuses.len() {
        reward += reward_factors.suit_streak_bonuses[suit_streak] * computed_ticket_price * growth_factor;
    }
    if value_streak > 1 && value_streak < reward_factors.value_streak_bonuses.len() {
        reward += reward_factors.value_streak_bonuses[value_streak] * computed_ticket_price * growth_factor;
    }
    
    let exact_match = (0..4).all(|i| {
        let (ticket_suit, ticket_value) = ticket_cards[i];
        let (winning_suit, winning_value) = winning_cards[i];
        ticket_suit == winning_suit && ticket_value == winning_value
    });
    
    if exact_match {
        reward += reward_factors.jackpot_percentage * accumulated_prize_pool as f64;
    }
    
    msg!("Calculated reward: {}", reward);

    let num_tickets_reward = calculate_num_tickets_reward(
        amount_payed, 
        ticket_price, 
        lottery_type,
        reward_factors.max_boost,
        reward_factors.curvature
    );

    msg!("Calculated num_tickets_reward: {}", num_tickets_reward);

    Ok(if lottery_type == PAY_LOTTERY_TYPE {
        ((reward.round()) as u64) * num_tickets_reward
    } else {
        amount_payed + (((reward.round()) as u64) * num_tickets_reward)
    })
}

pub fn claim_prize_for_combination(
    ctx: Context<ClaimLotteryPrizeForCombination>, 
    _lottery_id: u64, 
    _combination: String
) -> Result<()> {
    msg!("Claiming prize for combination");
    let clock = Clock::get()?;
    let lottery_state = &mut ctx.accounts.lottery_state;
    let ticket = &mut ctx.accounts.ticket;
    let accumulated_prize_pool = if lottery_state.lottery_type == PAY_LOTTERY_TYPE {
        lottery_state.accumulated_prize_pool
    } else {
        lottery_state.initial_prize_pool
    };

    require!(
        lottery_state.winning_combination.is_some(), 
        ErrorCode::WinningCombinationNotSetYet
    );
    require!(
        lottery_state.lottery_end_datetime < clock.unix_timestamp, 
        ErrorCode::LotteryNotFinished
    );
    require!(!ticket.is_claimed, ErrorCode::TicketAlreadyClaimed);
    require!(!lottery_state.is_closed, ErrorCode::LotteryClosed);

    let prize = calculate_prize(
        lottery_state.winning_combination.as_ref().unwrap(),
        &ticket.combination,
        lottery_state.lottery_type,
        ticket.amount,
        lottery_state.initial_prize_pool,
        accumulated_prize_pool,
        lottery_state.min_tokens_per_participant,
        &ctx.accounts.lottery_reward_factors
    )?;

    if prize > 0 {
        let lottery_id_bytes = lottery_state.lottery_id.to_le_bytes();
        let scaled_amount = instructions::utils::get_scaled_amount(
            prize,
            ctx.accounts.token_mint.decimals
        )?;

        msg!("Scaled amount {}", scaled_amount);
        msg!("Lottery Token account amount {}", ctx.accounts.lottery_token_account.amount);

        let not_enough_funds = scaled_amount > ctx.accounts.lottery_token_account.amount;
        
        let pda_seeds: &[&[u8]] = if not_enough_funds {
            &[b"treasury"]
        } else {
            &[b"lottery_token_account", &lottery_id_bytes]
        };
        
        let pda_bump: u8 = if not_enough_funds {
            ctx.bumps.treasury_token_account
        } else {
            ctx.bumps.lottery_token_account
        };
        
        let pda_owned_token_account = if not_enough_funds {
            &ctx.accounts.treasury_token_account
        } else {
            &ctx.accounts.lottery_token_account
        };
        
        let _ = pda_owned_token_accounts::withdraw(
            scaled_amount,
            pda_seeds,
            pda_bump,
            pda_owned_token_account,
            &ctx.accounts.payer_token_account,
            &ctx.accounts.token_mint,
            &ctx.accounts.token_program,
        );
        
        
    }
    ticket.is_claimed = true;
    Ok(())
}

pub fn close(ctx: Context<CloseLottery>, _lottery_id: u64) -> Result<()> {
    msg!("Closing lottery");
    let lottery_state = &mut ctx.accounts.lottery_state;
    let clock = Clock::get()?;
    require!(
        lottery_state.winning_combination.is_some(),
        ErrorCode::WinningCombinationNotSetYet
    );
    require!(
        lottery_state.lottery_end_datetime < clock.unix_timestamp,
        ErrorCode::LotteryNotFinished
    );
    require!(
        !lottery_state.is_closed, 
        ErrorCode::LotteryClosed
    );
    require!(
        lottery_state.lottery_end_datetime + LOTTERY_CLOSE_TIME_BUFFER < clock.unix_timestamp,
        ErrorCode::LotteryNotReadyToBeClosed
    );

    let tokens_left = ctx.accounts.lottery_token_account.amount;
    let lottery_id_bytes = lottery_state.lottery_id.to_le_bytes();

    let _ = pda_owned_token_accounts::withdraw(
        tokens_left,
        &[b"lottery_token_account", &lottery_id_bytes],
        ctx.bumps.lottery_token_account,
        &ctx.accounts.lottery_token_account,
        &ctx.accounts.treasury_token_account,
        &ctx.accounts.token_mint,
        &ctx.accounts.token_program,
    );

    lottery_state.is_closed = true;
    msg!("Lottery with id {} closed successfully", lottery_state.lottery_id);
    Ok(())
}