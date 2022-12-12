use std::str::FromStr;

use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{alpha1, char, digit1, multispace0, multispace1, one_of},
  combinator::{cut, map, map_res, opt},
  error::{context, VerboseError},
  multi::many0,
  sequence::{delimited, preceded, terminated, tuple},
  IResult, Parser,
};



#[derive(Debug)]
enum Instruction {
  Noop,
  AddX(isize)
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
          return Ok(Instruction::Noop);
        }
      
        let mut splits = s.split(" ");
        let addx = splits.next().unwrap();
        if addx == "addx" {
          let num = splits.next().unwrap();
          let num = num.parse::<isize>().map_err(|_| "Not a number".to_owned())?;

          return Ok(Instruction::AddX(num));
        }

        return Err("Not a valid ".to_owned());
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::{HashMap, HashSet}, ffi::IntoStringError};

    use super::*;


  #[test]
  fn adventofcode1() {

    let mut vec: Vec<isize> = vec![1];

    let instuctions = include_str!("input.txt")
      .lines()
      .filter_map(|s| Instruction::from_str(s).ok())
      .flat_map(|i| {
        match i {
            Instruction::Noop => vec![Instruction::Noop],
            Instruction::AddX(x) => vec![Instruction::Noop, Instruction::AddX(x)],
        }
      });

      for instruction in instuctions {
        let prev = vec.last().unwrap().clone();
        match instruction {
            Instruction::Noop => vec.push(prev),
            Instruction::AddX(x) => vec.push(prev + x),
        }
      }

      //println!("{vec:?}");
      let mut acc = 0isize;
      [20isize, 60, 100, 140, 180, 220].iter().for_each(|i| {
        let index = (*i - 1) as usize;
        println!("At {} : {}", i, vec[index]);
        acc = acc + i * vec[index];
      });
      println!("Result : {}", acc);
      let mut crt = [["."; 40]; 6];

      for (i, v) in vec.iter().enumerate() {
        let y = i / 40;
        let x = i - y * 40;

        println!("Pos Check ({x},{y}) ({i}) curr : ({v})");

        let range = {
          if x == 0 {
            0..=1
          }else {
            x-1..=x+1
          }
        };
        
        let v = v.clone();
        //print!("Checking {} -> {:?}", v, range);
        if v.abs() == v {
          let v = v as usize;
          if range.contains(&v) {
            //println!(" PRINT");
            crt[y][x] = "#";
          }else {
            //println!(" NO PRINT")
          }
        }else {
          //println!(" NO PRINT (negetive) ");
        }
        
      }


      println!("CRT");
      for i in crt {
        println!("{}", i.join(""));
      }

      
  }
}