use std::cmp::Ordering;


///
/// # Description
/// 数据对
///
#[derive(Debug, Eq)]
pub struct Pair<T1: Ord, T2: Ord> {
  pub first: T1,
  pub second: T2,
}

impl<T1: Ord, T2: Ord> Ord for Pair<T1, T2> {
  fn cmp(&self, other: &Self) -> Ordering {
    let delta = self.first.cmp(&other.first);
    if delta != Ordering::Equal {
      return delta
    }
    return self.second.cmp(&other.second)
  }
}

impl<T1: Ord, T2: Ord> PartialOrd for Pair<T1, T2> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

impl<T1: Ord, T2: Ord> PartialEq for Pair<T1, T2> {
  fn eq(&self, other: &Self) -> bool {
    self.first == other.first && self.second == other.second
  }
}
