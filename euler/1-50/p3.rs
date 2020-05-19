//!
//! # Problem Description
//! The prime factors of 13195 are 5, 7, 13 and 29.
//! What is the largest prime factor of the number 600851475143 ?
//!
//! @see https://projecteuler.net/problem=3
//!
//! # 题目描述
//! 找出 600851475143 最大素因子
//!
//! # 题解
//!
//! # 答案
//! 6857
//!
fn main() {
  let mut n = 600851475143;
  let square_root_of_n: i64 = ((n as f64).sqrt() + 10f64).ceil() as i64;
  let mut answer = 0;

  for x in 1..square_root_of_n {
    if x > n {
      break;
    }
    if n % x == 0 {
      n /= x;
      answer = x;
    }
  }
  println!("{}", answer)
}
