////////////////////////////////////////////////////////////
///                 Treasury constants                   ///
////////////////////////////////////////////////////////////

pub const TREASURY_INITIAL_AMOUNT: u64 = 300_000_000;

////////////////////////////////////////////////////////////
///                 Six Month Cliff Constants            ///
////////////////////////////////////////////////////////////

pub const SIX_MONTHS: i64 = 15768000; // 6 months in unix timestamp
pub const SIX_MONTHS_TRANSFERS_PER_PERIOD: u64 = 2;
pub const SIX_MONTH_CLIFF_AMOUNT: u64 = 50_000_000;

////////////////////////////////////////////////////////////
///                 Lottery Constants                    ///
////////////////////////////////////////////////////////////

pub const VALID_SUITS: [char; 4] = ['S', 'C', 'H', 'W'];
pub const VALID_VALUES: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
pub const COMBINATION_LENGTH: usize = 8;
pub const PAY_LOTTERY_TYPE: u8 = 0;
pub const LOCK_LOTTERY_TYPE: u8 = 1;
pub const LOTTERY_CLOSE_TIME_BUFFER: i64 = 10*24*60*60; // 10 days

////////////////////////////////////////////////////////////
///         Lottery Reward Factors  (Initial values)     ///
////////////////////////////////////////////////////////////

pub const INITIAL_REWARD_FULL_MATCH: f64 = 1.00;
pub const INITIAL_REWARD_SUIT_MATCH: f64 = 0.30;
pub const INITIAL_REWARD_VALUE_MATCH: f64 = 0.50;
// (index 0 and 1 are unused, index 2-4 represent streak lengths)
pub const INITIAL_SUIT_STREAK_BONUSES: [f64; 5] = [0.0, 0.0, 0.25, 0.60, 1.20];
pub const INITIAL_VALUE_STREAK_BONUSES: [f64; 5] = [0.0, 0.0, 0.50, 1.20, 2.20];
pub const INITIAL_JACKPOT_PERCENTAGE: f64 = 0.20;
pub const INITIAL_MAX_BOOST: f64 = 0.55;
pub const INITIAL_CURVATURE: f64 = 0.9;
pub const INITIAL_LOCK_DIVIDER: f64 = 50.00;