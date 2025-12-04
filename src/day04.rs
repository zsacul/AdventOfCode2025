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
        let  hash = tools::get_hash_table(input);
     
        Data 
        {
            hash,
            visited : HashSet::new(),
            dx : input[0].len(),
            dy : input.len(),

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
        for y in 0..self.dy
        {
            for x in 0..self.dx
            {        
                let mut c=0;
                let  o = Vec2::new(x as i64,y as i64);
                for p in Vec2::around8(&o)
                {
                    if self.get(p)=='@'
                    {
                        c+=1;
                    };
                }
                if c<4 && self.get(o)=='@'
                {
                    
                    self.visited.insert(o);
                }
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
    let mut res = 0;    
    
    loop {
        d.visited.clear();
        let c = d.count();
        if c==0 {
            break;
        }

        for p in d.visited.iter() {
            d.hash.insert(*p, '.');
        }
        res+=c;
    }
    res as usize
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
