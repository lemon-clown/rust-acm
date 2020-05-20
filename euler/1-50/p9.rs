//!
//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//!   $$a^2 + b^2 = c^2$$
//!
//! For example, $3^2 + 4^2 = 9 + 16 = 25 = 5^2$.
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//! Find the product abc.
//!
//! @see https://projecteuler.net/problem=9
//!
//! # 题目描述
//! 找到一组毕达哥拉斯数 $a,b,c$，满足 $a^2+b^2=c^2$ 且 $a + b + c = 1000$
//!
//! # 题解
//! 暴力枚举
//! 因为 $c^2 = (a^2+b^2) \leqslant (a+b)^2$，故有 $c <= a + b$，
//! 又显然 $c >= a$ 且 $c >= b$，故可以仅枚举满足如下条件的数对
//!   - $a \in [0, 500)$,
//!   - $b \in [a, 500)$,
//!   - $c = 1000 - a - b$,
//! 并仅判断 $a^2 + b^2 = c^2$ 即可
//!
//! # 答案
//! 31875000
//!

fn main() {
  let mut answer = 0;

  for x in 0..500 {
    for y in x..500 {
      let z = 1000 - x - y;
      if x * x + y * y == z * z {
        answer = x * y * z;
        println!("x: {}, y: {}, z: {}, answer: {}", x, y, z, answer);
      }
    }
  }

  println!("answer: {}", answer);
}
