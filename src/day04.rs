use std::collections::{HashMap, HashSet};
use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Data 
{
    hash    : HashMap<Vec2,char>,   
    visited : HashSet<Vec2>,
    dx      : usize,
    dy      : usize,
}

impl Data {
    fn new(input: &[String]) -> Self 
    {
        let hash = tools::get_hash_table(input);
     
        Data 
        {
            hash,
            visited : HashSet::new(),
            dx      : input[0].len(),
            dy      : input.len(),
        }
    }

    #[allow(unused)]
    fn print_hash(&self,vis:HashSet<Vec2>)
    {
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let p = Vec2::new(x as i64,y as i64);                        
                let c = *self.hash.get(&p).unwrap_or(&'.');
               
                if vis.contains(&p)
                {
                    print!("O");
                }
                  else 
                {
                    print!("{}", c);
                }
            }
            println!();
        }
        println!();
    }

    fn count(&mut self)->usize
    {        
        for (x,y) in tools::get_2d_iter(0,self.dx,0,self.dy)
        {        
            let o = Vec2::new(x as i64,y as i64);
            
            if self.get(o)=='@' && 
               Vec2::around8(&o)
                   .iter()
                   .filter(|&p| self.get(*p)=='@')
                   .count()<4
                   {
                       self.visited.insert(o);
                   }
        }            

        self.visited.len()
    }

    fn get(&self,p:Vec2)->char
    {
        *self.hash.get(&p).unwrap_or(&'.')
    }
}

pub fn part1(data:&[String])->usize
{    
    Data::new(data).count()
}

pub fn part2(data:&[String])->usize
{
    let mut d = Data::new(data);
    let mut res = d.count();    
    
    while !d.visited.is_empty()
    {
        for p in d.visited.iter() { d.hash.insert(*p, '.'); }
        d.visited.clear();
        res+= d.count();
    }
    res
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day4");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "..@@.@@@@.".to_string(),
        "@@@.@.@.@@".to_string(),
        "@@@@@.@.@@".to_string(),
        "@.@@@@..@.".to_string(),
        "@@.@@@@.@@".to_string(),
        ".@@@@@@@.@".to_string(),
        ".@.@.@.@@@".to_string(),
        "@.@@@.@@@@".to_string(),
        ".@@@@@@@@.".to_string(),
        "@.@.@@@.@.".to_string(),
    ];
    
    assert_eq!(part1(&v),13);
}

#[test]
fn test2()
{
    let v = vec![
        "..@@.@@@@.".to_string(),
        "@@@.@.@.@@".to_string(),
        "@@@@@.@.@@".to_string(),
        "@.@@@@..@.".to_string(),
        "@@.@@@@.@@".to_string(),
        ".@@@@@@@.@".to_string(),
        ".@.@.@.@@@".to_string(),
        "@.@@@.@@@@".to_string(),
        ".@@@@@@@@.".to_string(),
        "@.@.@@@.@.".to_string(),
    ];
    
    assert_eq!(part2(&v),43);
}
