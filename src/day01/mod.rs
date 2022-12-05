#[cfg(test)]
mod tests {
    use std::{io::{BufRead, BufReader}, fs::File};

    #[derive(Eq, Debug, PartialEq)]
    struct Elf {
      food: Vec<i32>
    }

    impl PartialOrd for Elf {
      fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.total().partial_cmp(&other.total())
      }
    }

    impl Ord for Elf {
      fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total().cmp(&other.total())
      }
    }

    impl Elf {
      pub fn new() -> Self {
        Elf {
          food: vec![]
        }
      }

      pub fn total(&self) -> i32 {
        self.food.iter().sum()
      }

      pub fn push(&mut self, calories: i32) {
        self.food.push(calories);
      }
    }

    #[test]
    fn adventofcode1() {
        let file = File::open("./src/day01/input.txt");
        let file = file.unwrap();
        let lines = BufReader::new(file).lines();

        let mut elves: Vec<Elf> = vec![];
        let mut currentElf = Elf::new();

        for line in lines  {
          if let Ok(line) = line {
            let calories = line.parse::<i32>();

            match calories {
                Ok(cal) => currentElf.push(cal),
                Err(_) => {
                  elves.push(currentElf);
                  currentElf = Elf::new();
                },
            }
          }            
        }

        let max = elves.iter().max();
        println!("{:?} = {}", max.unwrap(), max.unwrap().total());
      
    }

    #[test]
    fn adventofcode2() {
      let file = File::open("./src/day01/input.txt");
      let file = file.unwrap();
      let lines = BufReader::new(file).lines();

      let mut elves: Vec<Elf> = vec![];
      let mut currentElf = Elf::new();

      for line in lines  {
        if let Ok(line) = line {
          let calories = line.parse::<i32>();

          match calories {
              Ok(cal) => currentElf.push(cal),
              Err(_) => {
                elves.push(currentElf);
                currentElf = Elf::new();
              },
          }
        }            
      }

      elves.sort();
      elves.reverse();
      let max_elves = elves.iter().take(3);
      let max_of_3: i32 = max_elves.map(|e| e.total()).sum();
      println!("{:?}", max_of_3);
    
  }
}