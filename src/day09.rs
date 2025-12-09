use std::collections::HashMap;
use super::tools;        
use super::vec2::Vec2;

#[derive(Debug)]
struct Data {
      pairs: Vec<Vec2>,
      map: HashMap<Vec2,char>,
}

impl Data {
    fn new(input: &[String]) -> Self {
        
        
        let pairs = input.iter()
            .map(|line| {
                {
                    let s = tools::split_to_usize(line,",");                
                    Vec2::new(s[0] as i64,s[1] as i64)
                }
            })
            .collect();
        

        Data {
            pairs,
            map: HashMap::new(),
        }
    }


    fn get_pos(line:&[usize])->HashMap<usize,usize>
    {
        line.iter()
            .enumerate()
            .map(|(id,v)| (*v,id))
            .collect()
    }


    fn count1(&self)->usize
    {
        let mut res = 0;
        for i in 0..self.pairs.len()
        {
            for j in i+1..self.pairs.len()
            {
                let a = &self.pairs[i];
                let b = &self.pairs[j];

                let area = ((b.x - a.x).abs()+1) * ((b.y - a.y).abs()+1);
                res = res.max(area);
            }
        }
        res as usize
   
    }

    fn count2(&self)->usize
    {
        0    
    }

}

pub fn part1(data:&[String])->usize
{
    Data::new(data).count1()
}

pub fn part2(data:&[String])->usize
{
    Data::new(data).count2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day9");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "7,1".to_string(),
        "11,1".to_string(),
        "11,7".to_string(),
        "9,7".to_string(),
        "9,5".to_string(),
        "2,5".to_string(),
        "2,3".to_string(),
        "7,3".to_string(),
    ];
    assert_eq!(part1(&v),50);
}

#[test]
fn test2()
{
    let v = vec![
        "7,1".to_string(),
        "11,1".to_string(),
        "11,7".to_string(),
        "9,7".to_string(),
        "9,5".to_string(),
        "2,5".to_string(),
        "2,3".to_string(),
        "7,3".to_string(),
    ];
    assert_eq!(part2(&v),150);
}
