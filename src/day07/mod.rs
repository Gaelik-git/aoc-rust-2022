use std::{str::FromStr, collections::HashMap};
use eyre::{Result, Context, eyre};

use nom::{
  IResult,
  bytes::complete::{tag, take_while_m_n, take},
  combinator::map_res,
  sequence::tuple, error::ParseError
};

#[derive(Debug, Eq, PartialEq)]
enum Command {
  Cd(String), 
  Ls,
  LsResult(LsResult)
}

#[derive(Debug, Eq, PartialEq)]
enum LsResult {
  File(String, usize),
  Directory(String)
}


#[derive(PartialEq, Eq, Debug)]
enum Entity {
  File(usize),
  Directory(Map)
}

impl Entity {

  fn get_size(&self) -> usize {
    match self {
        Entity::File(size) => *size,
        Entity::Directory(map) => map.iter().map(|e| e.1.get_size()).sum(),
    }
  }
}

type Map = HashMap<String, Entity>;

impl FromStr for Command {
    type Err = eyre::Report;

    fn from_str(input: &str) -> Result<Self> {
      let res: IResult<&str, &str> = tag("$ ")(input);
      if res.is_ok() {
        let (left, _) = res.map_err(|_| eyre!("Not starting with $"))?;
      
        let res: IResult<&str, &str> = take(2usize)(left);
        let (left, found) = res.map_err(|_| eyre!("not followed by command"))?;
        
        match found {
          "ls" => return Ok(Command::Ls),
          "cd" => {
            let path = left.trim();
            return Ok(Command::Cd(path.to_owned()));
          },
          _ => unreachable!()
        }
      }else {
        let splits: Vec<&str> = input.splitn(2, " ").collect();
        let p1 = splits[0];
        let p2 = splits[1];

        match (p1, p2) {
          ("dir", _) => return Ok(Command::LsResult(LsResult::Directory(p2.to_owned()))),
          _ => return Ok(Command::LsResult(LsResult::File(p2.to_owned(), p1.parse::<usize>().unwrap())))
        }
      }
      unreachable!()
    }
}



#[cfg(test)]
mod tests {
    use std::str::Lines;

    use super::*;


  #[test]
  fn adventofcode1() -> Result<()> {

    let mut inputs = include_str!("input.txt").lines();

    let mut state: Map = HashMap::new();
    apply_to_state(&mut state, &mut inputs);

    let mut all_directories: Vec<&Entity> = vec![];
    flatten(&state, &mut all_directories);

    let total_size_folder: usize = all_directories
      .into_iter()
      .filter(|&e| match e {
        Entity::File(_) => false,
        Entity::Directory(_) => true,
      })
      .map(|el| el.get_size())
      .filter(|s| *s <= 100000usize )
      .sum();

    dbg!(total_size_folder);

    Ok(())
  }

  #[test]
  fn adventofcode2() -> Result<()> {

    let mut inputs = include_str!("input.txt").lines();

    let mut state: Map = HashMap::new();
    apply_to_state(&mut state, &mut inputs);

    let current_used_space = state.iter().next().unwrap().1.get_size();
    dbg!(current_used_space);

    let free_space = 70000000 - current_used_space;
    let space_to_free = 30000000 - free_space;

    let mut all_directories: Vec<&Entity> = vec![];
    flatten(&state, &mut all_directories);

    let size_of_folder_to_delete: usize = all_directories
      .into_iter()
      .filter(|&e| match e {
        Entity::File(_) => false,
        Entity::Directory(_) => true,
      })
      .map(|el| el.get_size())
      .filter(|s| *s >= space_to_free )
      .min()
      .unwrap();

    dbg!(size_of_folder_to_delete);

    Ok(())
  }

  fn flatten<'a>(state: &'a Map, acc:&mut Vec<&'a Entity> ) {

    for (_, val) in state {
      acc.push(val);
    }

    for (_, val) in state {
      if let Entity::Directory(val) = val {
        flatten(val, acc);
      }
    }

  }

  fn apply_to_state<'a>(state: &'a mut Map, lines: &mut Lines)-> () {

    while let Some(input) = lines.next() {
      let input = Command::from_str(input).unwrap();
      match input {
        Command::LsResult(e) => {
          match e {
            LsResult::File(n, s) => state.insert(n.to_owned(), Entity::File(s)),
            LsResult::Directory(n) => state.insert(n.to_owned(), Entity::Directory(HashMap::new())),
          };
        },
        Command::Cd(place) => {
          match place.as_str() {
            ".." => return (),
            other => {
              if !state.contains_key(other) {
                state.insert(other.to_owned(), Entity::Directory(HashMap::new()));
              }
              let new_state = state.get_mut(&other.to_owned()).unwrap();
              if let Entity::Directory(el) = new_state {
                apply_to_state(el, lines);
              }
            },
          }
        },
        Command::Ls => (),
    };
    }
    
    

    return ();
  }


  #[test]
  fn test_parsing_command() {

    assert_eq!(Command::from_str("$ ls").unwrap(), Command::Ls);
    assert_eq!(Command::from_str("$ cd /").unwrap(), Command::Cd("/".to_owned()));
    assert_eq!(Command::from_str("$ cd ..").unwrap(), Command::Cd("..".to_owned()));

    assert_eq!(Command::from_str("4060174 j").unwrap(), Command::LsResult(LsResult::File("j".to_owned(),4060174)));
    assert_eq!(Command::from_str("dir truc").unwrap(), Command::LsResult(LsResult::Directory("truc".to_owned())));
    assert_eq!(Command::from_str("$ cd test").unwrap(), Command::Cd("test".to_owned()));

  }

}