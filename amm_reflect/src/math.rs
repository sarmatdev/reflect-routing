use anyhow::anyhow;

const BASE_AMOUNT: u128 = 100_000000;

pub fn calculate_exact_in(amount: u64, rate: u64) -> anyhow::Result<u64> {
    let out_amount = (amount as u128)
        .checked_mul(rate as u128)
        .ok_or_else(|| anyhow!("Overflow in quote calculation"))?
        .checked_div(BASE_AMOUNT)
        .ok_or_else(|| anyhow!("Division error in quote calculation"))?;

    Ok(out_amount as u64)
}

pub fn calculate_exact_out(amount: u64, rate: u64) -> anyhow::Result<u64> {
    let in_amount = (amount as u128)
        .checked_mul(BASE_AMOUNT)
        .ok_or_else(|| anyhow!("Overflow in quote calculation"))?
        .checked_add(rate as u128 - 1) // Round up
        .ok_or_else(|| anyhow!("Overflow in quote calculation"))?
        .checked_div(rate as u128)
        .ok_or_else(|| anyhow!("Division error in quote calculation"))?;

    Ok(in_amount as u64)
}
