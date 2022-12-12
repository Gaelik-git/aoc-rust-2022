#[cfg(test)]
mod tests {


  #[derive(Debug)]
  struct State {
    inner: Vec<Vec<char>>
  }

  impl State {

    pub fn result(&self) -> String{
      let mut res = String::new();

      for elems in &self.inner {
          res.push(*elems.last().unwrap());
      }

      res
    }

    pub fn moving_by_one(&mut self, action: &Action) {

      println!("Moving {} from {} to {}", action.nbr, action.from, action.to);
      for i in 0..action.nbr {
        let t = self.inner[action.from as usize - 1].pop().unwrap();
        self.inner[action.to as usize - 1].push(t);
      }
    }

    pub fn moving(&mut self, action: &Action) {

      println!("Moving {} from {} to {}", action.nbr, action.from, action.to);
      let i = action.from as usize - 1;
      let old_size = self.inner[i].len();
      let new_size = old_size - action.nbr;
      //println!("{old_size} -> {new_size}");

      let mut elems = self.inner[i][new_size..old_size].to_vec();
      self.inner[i].truncate(new_size);
      self.inner[action.to as usize - 1].append(&mut elems);
      //println!("Result {:#?}", self);

    }

    pub fn nbr_column(&self) -> usize {
      self.inner.len()  
    }

    pub fn new(inputs: Vec<Vec<Option<char>>>) -> Self {
      let nbr_col = inputs[0].len();
      let mut inner: Vec<Vec<char>> = vec![vec![]; nbr_col];

      let mut inputs = inputs.clone();
      inputs.reverse();

      for line in inputs {
        for (index, val) in line.iter().enumerate() {
          if let Some(c) = val {
            inner[index].push(*c);
          }
        }
      }

      State { inner }
    }
  }

  #[derive(Debug)]
  struct Action {
    nbr: usize,
    from: u8,
    to: u8
  }

  impl From<&str> for Action {
    fn from(input: &str) -> Self {
      let mut vals = input.split(' ')
        .filter_map(|s| s.parse::<u8>().ok());
      Self {
        nbr: vals.next().unwrap() as usize,
        from: vals.next().unwrap(),
        to: vals.next().unwrap(),
    }
    }
}

  #[derive(Debug, PartialEq, Eq)]
  enum Possibility {
    Empty,
    Present(char),
    End
  }

  fn valid_line(input: &str) -> Option<Vec<Option<char>>> {
    let mut res = vec![];
    let mut input = input.chars();
    
    while let Some(v) = match (input.next(), input.next(), input.next(), input.next()) {
      (Some(' '), Some(' '), Some(' '), _) => Some(Possibility::Empty),
      (Some('['), Some(c), Some(']'), _) => Some(Possibility::Present(c)),
      (None, None, None, None) => None,
      _ => return None,
  }{
    match v {
        Possibility::Empty => res.push(None),
        Possibility::Present(c) =>res.push(Some(c)),
        Possibility::End => (),
    }
  }
    Some(res)
  }

  #[test]
  fn adventofcode1() {
    
    let mut lines = include_str!("input.txt").lines();

    let mut state = State::new(lines.clone().map_while(|l| valid_line(l)).collect());
    let actions: Vec<Action> = lines.skip(state.nbr_column() + 1).map(|s| s.into()).collect::<Vec<_>>();
    //println!("{actions:?}");
    println!("before {state:?}");

    for action in actions {
      state.moving_by_one(&action);
      //println!("action {action:?}");
      //println!("current {state:#?}");
  }

    println!("after {state:?}");

    println!("{:?}", state.result());

  }

  #[test]
  fn adventofcode2() {
    
    let mut lines = include_str!("input.txt").lines();

    let mut state = State::new(lines.clone().map_while(|l| valid_line(l)).collect());
    let actions: Vec<Action> = lines.skip(state.nbr_column() + 1).map(|s| s.into()).collect::<Vec<_>>();
    //println!("{actions:?}");
    println!("before {state:?}");

    for action in actions {
      state.moving(&action);
      //println!("action {action:?}");
      //println!("current {state:#?}");
  }

    println!("after {state:?}");

    println!("{:?}", state.result());

  }
}