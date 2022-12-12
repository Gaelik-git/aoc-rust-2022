#[cfg(test)]
mod tests {

  #[derive(Debug)]
  struct Signal(String);

  impl Signal {
    pub fn new(string: String) -> Self {
      Signal(string)
    }

    pub fn first_marker(&self, size: usize) -> usize {
      let mut chars = self.0.chars();
      let mut index = 0;
      
      let mut window = vec![];
      
      for _ in 0..size {
        window.push(chars.next().unwrap());
        index += 1;
      }


      while duplicate_in(&window) {
        window.remove(0);
        window.push(chars.next().unwrap());
        index += 1;
      }

      return index;

    }
  }

  fn duplicate_in(window: &Vec<char>) -> bool {
    println!("looking in {window:?}");
    for i in 0..window.len() - 1 {
      for j in i+1..window.len() {
        println!("comparing {i} - {j}");
        if window[i] == window[j] {
          println!("Duplicate found");
          return true;
        }
      }
    }
    println!("No duplicate");
    return false;
  }

  #[test]
  fn adventofcode1() {
    
    let streams: Vec<_> = include_str!("input.txt")
      .lines()
      .map(|s| Signal::new(s.to_owned()))
      .map(|s| s.first_marker(4))
      .collect();

    println!("{:?}", streams);

  }

  #[test]
  fn adventofcode2() {
    
    let streams: Vec<_> = include_str!("input.txt")
      .lines()
      .map(|s| Signal::new(s.to_owned()))
      .map(|s| s.first_marker(14))
      .collect();

    println!("{:?}", streams);

  }
}