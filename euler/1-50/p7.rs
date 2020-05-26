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
//! 由于素数的稀疏度不会很大（至少前100w），复杂度在 $O(n)$ 左右
//!
//! # 答案
//! 104743
//!

use rust_acm::algorithm::prime;

fn main() {
  let n = 10001 as usize;
  let mut max_n = (n * 8) as u64;

  let primes = loop {
    let ret = prime::sieve_primes(max_n);
    if ret.len() < n {
      max_n *= 2;
      continue;
    }
    break ret
  };
  let answer = primes[n - 1];
  println!("{}", answer)
}
