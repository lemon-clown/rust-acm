//!
//! 2520 is the smallest number that can be divided by each of the numbers
//! from 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of
//! the numbers from 1 to 20?
//!
//! @see https://projecteuler.net/problem=5
//!
//! # 题目描述
//! 求区间 [1,20] 中所有整数的的最小公倍数
//!
//! # 题解
//! 从 [1,20] 遍历 i，设之前处理得到集合 A $\{a1,a2,...,a_{i-1}\}$，
//! 那么令 x 等于 i 除去所有集合 A 中 i 能背起整除的数之后的剩余结果，
//! 并将 x 追加到集合 A 中，$即 a_i = x$。那么答案就是集合 A 的所有元素的乘积
//!
//! # 答案
//! 232792560
//!
fn main() {
  let mut factors = vec![];
  for x in 1..21 {
    let mut y = x;
    for e in factors.iter() {
      if y % e == 0 {
        y /= e;
      }
    }
    factors.push(y);
  }

  let mut answer = 1i64;
  for x in factors.iter() {
    answer *= x;
  }
  println!("{}", answer)
}
