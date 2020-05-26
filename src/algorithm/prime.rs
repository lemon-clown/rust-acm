//!
//! # Description
//! 筛选素数，筛出给定的数范围内的所有素数
//!
//! # Params
//! * `max_n`: 最大的数，超过此数的素数不被采用
//!
pub fn sieve_primes(max_n: u64) -> Vec<u64> {
  use std::collections::HashSet;
  let mut non_primes = HashSet::new();
  let mut primes = Vec::with_capacity((max_n / 20u64) as usize);

  let mut x: u64 = 2;
  let mut executed_times = 0;
  while x <= max_n {
    if !non_primes.contains(&x) {
      primes.push(x);
    }
    for e in primes.iter() {
      if  e * x > max_n  {
        break;
      }
      non_primes.insert(e * x);
      executed_times += 1;
      if x % e == 0 {
        break;
      }
    }
    x += 1;
  }
  println!("executed times: {}", executed_times);
  return primes;
}
