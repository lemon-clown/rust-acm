use crate::collection::pair::Pair;


///
/// # Description
/// 筛选素数，筛出给定的数范围内的所有素数
///
/// # Params
/// * `max_n`: 最大的数，超过此数的素数不被采用
///
pub fn sieve_primes(max_n: u64) -> Vec<u64> {
  use std::collections::HashSet;
  let mut non_primes = HashSet::new();
  let mut primes = Vec::with_capacity((max_n / 20u64) as usize);

  let mut x: u64 = 2;

  #[warn(unused_variables)]
  let mut _executed_times = 0;

  while x <= max_n {
    if !non_primes.contains(&x) {
      primes.push(x);
    }
    for e in primes.iter() {
      if e * x > max_n  {
        break;
      }
      non_primes.insert(e * x);
      _executed_times += 1;
      if x % e == 0 {
        break;
      }
    }
    x += 1;
  }
  // println!("executed times: {}", _executed_times);
  return primes;
}


///
/// # Description
/// 素因子分解
///
/// # Params
/// * `n`: 待分解的数
///
/// # Returns
/// 数对向量：数对中第一个值为素因子，第二个值为此素因子的幂次
///
pub fn calc_factorization(mut n: i32) -> Vec<Pair<i32, usize>> {
  let mut ret: Vec<Pair<i32, usize>> = vec![];
  let square_root_of_n: i32 = (n as f32).sqrt().ceil() as i32 + 1;
  for x in 2..square_root_of_n {
    let mut exponential = 0;
    while n % x == 0 {
      exponential += 1;
      n /= x;
    }
    if exponential > 0 {
      let factor = Pair {
        first: x,
        second: exponential,
      };
      ret.push(factor);
    }
  }

  if n > 1 {
    let factor = Pair {
      first: n,
      second: 1,
    };
    ret.push(factor);
  }
  return ret;
}
