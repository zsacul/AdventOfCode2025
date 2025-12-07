use std::collections::{HashMap, HashSet};
use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Data 
{
    hash    : HashMap<Vec2,char>,   
    visited : HashSet<Vec2>,
    dy      : usize,
    start   : Vec2,
}

impl Data {
    fn new(input: &[String]) -> Self 
    {
        let hash = tools::get_hash_table(input);
        let start = *hash.iter().find(|(_, &c)| c == 'S').unwrap().0;
     
        Data 
        {
            hash,
            visited : HashSet::new(),
            dy      : input.len(),
            start,
        }
    }

    fn count(&mut self)->usize
    {        
        let mut res = 0;
        let mut tachions = vec![self.start];

        while  !tachions.is_empty() 
        {
            let mut new_tachions = vec![];

            for t in tachions
            {               
                let p = Vec2::new(t.x, t.y + 1);

                if t.y+1 < self.dy as i64
                {                
                    if !self.visited.contains(&p)
                    {
                        if self.get(p)=='^'
                        {
                            let l = Vec2::new(p.x-1,p.y);
                            let r = Vec2::new(p.x+1,p.y);
                            self.visited.insert(l);
                            self.visited.insert(r);
                            new_tachions.push(l);
                            new_tachions.push(r);
                            res+=1;
                        }
                        else
                        {                            
                            self.visited.insert(p);
                            new_tachions.push(p);
                        }
                    }                
                }
            }

            tachions = new_tachions;            
            tachions.sort();
            tachions.dedup();
            
        }

        res
    }

    fn count2(&mut self)->usize
    {        
        let mut res = 0;
        let mut tachions = vec![self.start];
        let mut ways: HashMap<Vec2,usize> = HashMap::new();
        ways.insert(self.start,1);

        while  !tachions.is_empty() 
        {            
            let mut new_tachions = vec![];
            let mut new_ways: HashMap<Vec2,usize> = HashMap::new();

            res = ways.iter().map(|p|p.1).sum();

            for t in tachions
            {               
                let p = Vec2::new(t.x, t.y + 1);

                if t.y+1 < self.dy as i64
                {                
                    if self.get(p)=='^'
                    {
                        let l = Vec2::new(p.x-1,p.y);
                        let r = Vec2::new(p.x+1,p.y);
                        
                        self.visited.insert(l);
                        self.visited.insert(r);
                        new_tachions.push(l);
                        new_tachions.push(r);

                        let lw = *ways.get(&t).unwrap_or(&0);
                        let la  = *new_ways.get(&l).unwrap_or(&0);
                        let lb  = *new_ways.get(&r).unwrap_or(&0);
                        new_ways.insert(l, la + lw);
                        new_ways.insert(r, lb + lw);
                    }
                        else
                    {
                        self.visited.insert(p);
                        new_tachions.push(p);
                        let pw = *ways.get(&t).unwrap_or(&0);
                        let pa  = *new_ways.get(&p).unwrap_or(&0);
                        new_ways.insert(p, pa + pw);    
                    }

                }
            }

            tachions = new_tachions;            
            tachions.sort();
            tachions.dedup();
            
            ways = new_ways;
        }

        res
    }

    fn get(&self,p:Vec2)->char
    {
        *self.hash.get(&p).unwrap_or(&'.')
    }
}

fn part1(data:&[String])->usize
{    
    Data::new(data).count()
}

fn part2(data:&[String])->usize
{
    Data::new(data).count2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day7");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        ".......S.......".to_string(),
        "...............".to_string(),
        ".......^.......".to_string(),
        "...............".to_string(),
        "......^.^......".to_string(),
        "...............".to_string(),
        ".....^.^.^.....".to_string(),
        "...............".to_string(),
        "....^.^...^....".to_string(),
        "...............".to_string(),
        "...^.^...^.^...".to_string(),
        "...............".to_string(),
        "..^...^.....^..".to_string(),
        "...............".to_string(),
        ".^.^.^.^.^...^.".to_string(),
        "...............".to_string(),
    ];
    
    assert_eq!(part1(&v),21);
}

#[test]
fn test2()
{
    let v = vec![
        ".......S.......".to_string(),
        "...............".to_string(),
        ".......^.......".to_string(),
        "...............".to_string(),
        "......^.^......".to_string(),
        "...............".to_string(),
        ".....^.^.^.....".to_string(),
        "...............".to_string(),
        "....^.^...^....".to_string(),
        "...............".to_string(),
        "...^.^...^.^...".to_string(),
        "...............".to_string(),
        "..^...^.....^..".to_string(),
        "...............".to_string(),
        ".^.^.^.^.^...^.".to_string(),
        "...............".to_string(),
    ];
    
    assert_eq!(part2(&v),40);
}
