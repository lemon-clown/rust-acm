/**
 * Each new term in the Fibonacci sequence is generated by adding the previous
 * two terms. By starting with 1 and 2, the first 10 terms will be:
 *
 *    1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
 *
 * By considering the terms in the Fibonacci sequence whose values do not
 * exceed four million, find the sum of the even-valued terms.
 *
 * @see https://projecteuler.net/problem=2
 *
 * # 题目描述
 * 找出斐波那契数列中值不超过 4000000 且值为偶数的项的和
 *
 * # 题解
 * 暴力枚举
 *
 * # 答案
 * 4613732
 */
fn main () {
  let max_num = 4000000;
  let mut f1 = 1;
  let mut f2 = 2;
  let mut f3 = f1 + f2;
  let mut answer = f2;

  while f3 < max_num {
    f1 = f2;
    f2 = f3;
    f3 = f1 + f2;
    if (f3 & 1) == 0 {
      answer += f3;
    }
  }
  println!("{}", answer)
}
