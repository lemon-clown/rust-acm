//!
//! A palindromic number reads the same both ways. The largest palindrome made
//! from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.
//!
//! @see https://projecteuler.net/problem=4
//!
//! # 题目描述
//! 找到两个三位数乘积构成的回文数中最大的那个回文数
//!
//! # 题解
//! 暴力枚举
//!
//! # 答案
//! 906609
//!
fn main() {
  let mut answer = 0;
  for x in 100..1000 {
    for y in x..1000 {
      let z:i32 = x * y;
      let rz:i32 = {
        let mut cz:i32 = z;
        let mut v:i32 = 0;
        loop {
          if cz <= 0 {
            break
          }
          v = v * 10 + cz % 10;
          cz /= 10;
        }
        // return value
        v
      };
      if z == rz && answer < z {
        answer = z;
      }
    }
  }
  println!("{}", answer)
}
