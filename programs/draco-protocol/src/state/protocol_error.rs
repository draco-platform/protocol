use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Payer does not match protocol authority")]
    InvalidAuthority,

    #[msg("Not enough time has passed since the last transfer")]
    NotEnoughTimePassed,

    #[msg("Max transfers performed")]
    MaxTransfersPerformed,

    #[msg("Lottery is not started")]
    LotteryNotStarted,

    #[msg("Lottery is not finished")]
    LotteryNotFinished,

    #[msg("Lottery is finished")]
    LotteryFinished,

    #[msg("Lottery is closed")]
    LotteryClosed,

    #[msg("Invalid combination length")]
    InvalidCombinationLength,

    #[msg("Invalid combination suit")]
    InvalidCombinationSuit,

    #[msg("Invalid combination value")]
    InvalidCombinationValue,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Invalid lottery type")]
    InvalidLotteryType,

    #[msg("Invalid lottery start and end datetime")]
    InvalidLotteryStartEndDatetime,

    #[msg("Invalid initial prize pool")]
    InvalidInitialPrizePool,
    
    #[msg("Invalid min tokens per participant")]
    InvalidMinTokensPerParticipant,

    #[msg("Lottery is not finished")]
    CantCommitOnNotFinishedLottery,

    #[msg("Randomness already revealed")]
    RandomnessAlreadyRevealed,

    #[msg("Incorrect randomness account")]
    IncorrectRandomnessAccount,

    #[msg("Cannot reveal on not finished lottery")]
    CantRevealOnNotFinishedLottery,

    #[msg("Combination already set!")]
    CombinationAlreadySet,

    #[msg("Randomness not resolved")]
    RandomnessNotResolved,

    #[msg("Winning combination not set yet")]
    WinningCombinationNotSetYet,

    #[msg("Ticket already claimed")]
    TicketAlreadyClaimed,

    #[msg("Lottery not ready to be closed. Have to pass 10 days after the lottery end datetime")]
    LotteryNotReadyToBeClosed,

    #[msg("Aritmetic overflow")]
    ArithmeticOverflow,

    #[msg("Invalid airdrop start and end datetime")]
    InvalidAirdropStartEndDatetime,

    #[msg("Invalid airdrop supply")]
    InvalidAirdropSupply,

    #[msg("Airdrop has not started yet")]
    AirdropNotStarted,

    #[msg("Airdrop has ended")]
    AirdropEnded,

    #[msg("Airdrop supply exhausted")]
    AirdropSupplyExhausted,
}