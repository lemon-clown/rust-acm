//!
//! The sum of the squares of the first ten natural numbers is,
//! $$1^2 + 2^2 + ... + 10^2 = 385$$
//! The square of the sum of the first ten natural numbers is,
//! $$(1 + 2 + ... + 10)^2 = 55^2 = 3025$$
//!
//! Hence the difference between the sum of the squares of the first ten natural
//! numbers and the square of the sum is $3025 - 385 = 2640$.
//!
//! Find the difference between the sum of the squares of the first one hundred
//! natural numbers and the square of the sum.
//!
//! @see https://projecteuler.net/problem=6
//!
//! # 题目描述
//! 求 $\displaystyle \left(\sum_{i=1}^{100} i\right)^2 - \sum_{i=1}^{100} i^2$
//!
//! # 题解
//! 暴力枚举
//!
//! # 答案
//! 25164150
//!
fn main() {
  let n = 100;

  // sum of the squares of the first one hundred natural numbers
  let sum_of_square:i32 = {
    let mut s = 0;
    for x in 1..=n {
      s += x * x;
    }
    s
  };

  // square of sum of the first one hundred natural numbers
  let square_of_sum:i32 = {
    let s = (1 + n) * n / 2;
    s * s
  };

  let answer = square_of_sum - sum_of_square;
  println!("{}", answer)
}
