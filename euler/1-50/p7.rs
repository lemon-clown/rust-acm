//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
//! we can see that the 6th prime is 13.
//!
//! What is the 10 001st prime number?
//!
//! @see https://projecteuler.net/problem=7
//!
//! # 题目描述
//! 求从 2 开始的第 10001 个素数
//!
//! # 题解
//! 素数线性筛，由于只指定了素数个数无法直接使用线性筛，可以通过倍增上限来处理。
//! 由于素数的稀疏度不会很大（至少前100w），复杂度在 O(n) 左右
//!
//! # 答案
//! 104743
//!

fn main() {
  let n = 10001 as usize;
  let mut max_n = (n * 8) as u64;
  let primes = loop {
    let ret = sieve(n, max_n);
    if ret.len() < n {
      max_n *= 2;
      continue;
    }
    break ret
  };
  let answer = primes[primes.len() - 1];
  println!("{}", answer)
}

use std::collections::HashSet;
fn sieve(n: usize, max_n: u64) -> Vec<u64> {
  let mut primes = Vec::with_capacity(n);
  let mut non_primes = HashSet::new();

  let mut x: u64 = 2;
  let mut executed_times = 0;
  while x <= max_n {
    if !non_primes.contains(&x) {
      primes.push(x);
      if primes.len() >= n {
        break;
      }
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
  println!("execCount: {}", executed_times);
  return primes;
}
