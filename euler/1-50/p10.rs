//!
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! Find the sum of all the primes below two million.
//!
//! @see https://projecteuler.net/problem=10
//!
//! # 题目描述
//! 找到前两百万个素数的和
//!
//! # 题解
//! - 素数筛
//!
//! # 答案
//! 142913828922
//!
use rust_acm::algorithm::prime;

fn main() {
  let max_n = (200_0000) as u64;
  let primes = prime::sieve_primes(max_n);
  let answer = primes.iter()
    .fold(0, |acc, x| acc + x);
  println!("{}", answer)
}
