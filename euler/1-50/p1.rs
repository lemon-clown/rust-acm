//!
//! # Problem Description
//! If we list all the natural numbers below 10 that are multiples of 3 or 5,
//! we get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.
//!
//! @see https://projecteuler.net/problem=1
//!
//! # 题目描述
//! 计算 [0, 1000) 中所有能被 3 或 5 整除的整数的和
//!
//! # 题解
//! 暴力枚举
//!
//! # 答案
//! 233168
//!
fn main() {
  let mut answer = 0;
  for x in 0..1000 {
    if x % 3 == 0 || x % 5 == 0 {
      answer += x;
    }
  }
  println!("{}", answer)
}
