/*
We do not just store token vault balances in the pool struct because some protocols like Meteora
do not directly store real token vault balances and instead derive them from other balances such
as lp token balances for that pool and other values. Therefore, we instead use a trait and have
every AmmPool define the ability to calculate the real token vault balances from the data it has.
Each protocol stores the data necessary to calculate the real token vault balances in their own
structs and implements this trait.
*/

pub trait AmmPool {
  // The actual real amount of token A in the pool in the units of the token
  fn token_a_amount_units(&self) -> u64;
  // The actual real amount of token B in the pool in the units of the token
  fn token_b_amount_units(&self) -> u64;
}
