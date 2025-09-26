
use crate::state::protocol_error::ErrorCode;

pub fn get_scaled_amount(amount: u64, decimals: u8) -> Result<u64, ErrorCode> {
    let scaled_amount = amount
        .checked_mul(10_u64.pow(decimals as u32),)
        .ok_or(ErrorCode::ArithmeticOverflow);

    return scaled_amount;
}