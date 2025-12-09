use std::collections::HashMap;
use super::vec2::Vec2;
use super::tools;

#[derive(Debug)]
struct Data 
{
    hash    : HashMap<Vec2,char>,   
    dy      : i64,
    start   : Vec2,
}

impl Data {
    fn new(input: &[String]) -> Self 
    {
        let hash  = tools::get_hash_table(input);
        let start = tools::find_in_hash(&hash,'S');        
     
        Data 
        {
            hash,
            dy : input.len() as i64,
            start,
        }
    }

    fn count(&mut self)->usize
    {        
        let mut splits = 0;
        let mut tachions = vec![self.start];

        while !tachions.is_empty() 
        {
            let new_tachions: Vec<Vec2> = tachions.iter()
                                                  .flat_map(|t| 
                                                  {
                                                        let point = t.d();

                                                        if point.y < self.dy 
                                                        {
                                                          if self.get(point) == '^' 
                                                          {
                                                              splits += 1;
                                                              vec![point.l(), point.r()]
                                                          } 
                                                            else 
                                                          {
                                                              vec![point]
                                                          }
                                                        } 
                                                          else 
                                                        {
                                                          vec![]
                                                        }
                                                  }).collect();

            tachions = new_tachions;            
            tachions.dedup();            
        }

        splits
    }

    fn count2(&mut self)->usize
    {        
        let mut res = 0;
        let mut tachions = vec![self.start];
        let mut ways: HashMap<Vec2,usize> = HashMap::new();

        ways.insert(self.start,1);

        while !tachions.is_empty() 
        {            
            let mut new_tachions = vec![];
            let mut new_ways: HashMap<Vec2,usize> = HashMap::new();

            res = ways.iter().map(|p| p.1).sum();

            for t in tachions
            {               
                let p = t.d();

                if p.y < self.dy
                {                
                    if self.get(p)=='^'
                    {
                      let l = p.l();
                      let r = p.r();
                        
                      new_tachions.push(l);
                      new_tachions.push(r);

                      let   act =     *ways.get(&t).unwrap_or(&0);
                      let l_add = *new_ways.get(&l).unwrap_or(&0);
                      let r_add = *new_ways.get(&r).unwrap_or(&0);

                      new_ways.insert(l, act + l_add);
                      new_ways.insert(r, act + r_add);
                    }
                      else
                    {
                      new_tachions.push(p);

                      let act =     *ways.get(&t).unwrap_or(&0);
                      let add = *new_ways.get(&p).unwrap_or(&0);

                      new_ways.insert(p, act + add);
                    }
                }
            }

            tachions = new_tachions;
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

#[allow(unused)]
fn get_test_data()->Vec<String>
{
   vec![
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
    ]     
}

#[test]
fn test1()
{   
    assert_eq!(part1(&get_test_data()),21);
}

#[test]
fn test2()
{   
    assert_eq!(part2(&get_test_data()),40);
}
