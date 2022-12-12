#[cfg(test)]
mod tests {

  #[test]
  fn adventofcode1() {
    let lines = include_str!("input.txt").lines();

    let mut res = 0u64;

    for line in lines {
      let vec = line.chars()
        .collect::<Vec<_>>();

      let (p1, p2) = vec.split_at(vec.len() / 2);

      let common = p1.iter().filter(|&c| p2.contains(c)).next().unwrap();

      let val = match common {
            ('a'..='z') => *common as u8 - b'a' + 1  ,
            ('A'..='Z') => *common as u8 - b'A' + 27 ,
            _ => unreachable!(),
        };
      
      println!("{line:?}");
      println!("{common:?} - {val:?}");

      res += val as u64;
      
    }

    println!("{res:?}");
  }


  #[test]
  fn adventofcode2() {
    let mut lines = include_str!("input.txt").lines();

    let mut res = 0u64;

    while let (Some(p1), Some(p2), Some(p3)) = (lines.next(), lines.next(), lines.next()) {

      let common = p1.chars().filter(|&c| p2.contains(c) && p3.contains(c)).next().unwrap();
      let val = match common {
        ('a'..='z') => common as u8 - b'a' + 1  ,
        ('A'..='Z') => common as u8 - b'A' + 27 ,
        _ => unreachable!(),
      };
     res += val as u64;

    }

    println!("{res:?}");
  }
}
