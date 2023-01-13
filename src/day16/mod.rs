use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn adventofcode1() {
        
    let mut valves: HashMap<&str, isize> = Default::default();
    let mut directs : HashMap<&str, Vec<&str>> = Default::default();

    include_str!("input.txt").lines().for_each(|s| {
        let splits = s.splitn(10, " ").collect_vec();
        let name = splits[1];
        let flow = splits[4].split_once("=").unwrap().1.split_once(";").unwrap().0.parse::<isize>().unwrap();
        let targets = splits.last().unwrap().split(", ").collect_vec();

        valves.insert(name, flow);
        directs.insert(name, targets);
    });

    let mut graph : HashMap<&str, HashMap<&str, isize>> = Default::default();

    println!("valves : {valves:?}");
    println!("directs : {directs:?}");

    let no_flow_valves = valves.iter().filter(|(&n, &flow)| flow == 0 && n != "AA").map(|(&n, _)| n).collect_vec();
    println!("Stuck valves {no_flow_valves:?}");

    let indices: HashMap<&str, isize> = valves.iter().filter(|(&n, &flow)| flow != 0 || n == "AA").enumerate().map(|(i, (&name, _))| (name, (i + 1) as isize)).collect();
    println!("Flowing with index {indices:?}");


    for (&name, _) in valves.iter() {

        if valves[name] == 0 && name != "AA" {
            continue;
        }

        graph.insert(name, [(name, 0)].into());
        let mut queue = VecDeque::new();
        queue.push_back(name);

        while let Some(curr) = queue.pop_front() {
            for &next in directs[curr].iter() {
                if graph[name].contains_key(next)  {
                    continue;
                }
                let dist = graph[name][curr] + 1;

                graph.get_mut(name).unwrap().insert(next, dist);
                queue.push_back(next);
            }
        }
    }

    no_flow_valves.iter().for_each(|&n|{ graph.remove(n); });

    graph.iter_mut().for_each(|(_, map)| {
        no_flow_valves.iter().for_each(|&n|{ map.remove(n); });
    });

    let cache: HashMap<(&str, isize, i32), isize> = Default::default();

    struct Dfs<'a> {
        cache: HashMap<(&'a str, isize, i32), isize>,
        graph: HashMap<&'a str, HashMap<&'a str, isize>>,
        indices: HashMap<&'a str, isize>,
        valves: HashMap<&'a str, isize>,
        cache_used: usize,
        cache_filled: usize,
    }

    impl Dfs<'static> {
        fn dfs<'a, 'b>(&'a mut self, valve: &'static str, minutes: isize, bitmask: i32) -> isize {
            if self.cache.contains_key(&(valve, minutes, bitmask)) {
                self.cache_used += 1;
                return self.cache[&(valve, minutes, bitmask)];
            }

            let mut max_val = 0;
            for (&voisin, &dist) in self.graph[valve].clone().iter() {
                let bit = 1 << self.indices[voisin];

                if bitmask & bit != 0 {
                    continue;
                }
                
                let time_remaining = minutes - dist - 1;
                if time_remaining <= 0 {
                    continue;
                }
                let new_bit_mask = bitmask | bit;
                let new_val = self.dfs(voisin, time_remaining, new_bit_mask) + time_remaining * self.valves[voisin];
                max_val = max_val.max(new_val);
            }
            self.cache.insert((valve, minutes, bitmask), max_val);
            self.cache_filled += 1;

            max_val
        }
    }
    

    let mut dfs = Dfs {
        cache, graph, indices, valves, cache_filled: 0, cache_used: 0
    };

    let mut max = 0;

    let b = (1 << (dfs.indices.len() - 1)) - 1;
    dbg!(b);
    for i in 0..b/2 {
        let res = dfs.dfs("AA", 26, i) + dfs.dfs("AA", 26, b ^ i );
        max = max.max(res);
    }

    dbg!(dfs.cache_filled);
    dbg!(dfs.cache_used);
    dbg!(dfs.cache.len());
    dbg!(max);
    }

}
