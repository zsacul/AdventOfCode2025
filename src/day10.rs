use std::{collections::HashSet};
use super::tools;
use good_lp::solvers::highs::highs;
use good_lp::{Solution, SolverModel,variables, variable, Expression};

#[derive(Debug)]
struct World
{
    code    : u64,
    buttons : Vec<u64>,
    voltge  : Vec<u64>,
 }

impl World
{
    fn to_bin(s:String)->u64
    {
        s.split(",")
         .fold(0, |acc,v| acc | 1<<v.parse::<u64>().unwrap() )
    }

    fn new(s:&String)->World 
    {
        let t = tools::get_between(s,"[","]");
        let t = t.replace('.', "0");
        let t = t.replace('#', "1");

        let code = t.chars().rfold(0u64,|acc,c| (acc<<1) | (c as u8 - b'0') as u64);

        let butons = tools::get_between(s,"] "," {");

        let buttons:Vec<u64> = butons
                              .split(" ")
                              .map(|x| World::to_bin(tools::get_between(x, "(", ")") ))
                              .collect();

        let clicks = butons
                     .split(" ")
                     .map(|x|
                         {
                             tools::get_between(x, "(", ")")
                             .split(',')
                             .map(|y| y.parse::<usize>().unwrap() )
                             .collect::<HashSet<usize>>()
                         }                
                     )
                     .collect::<Vec<HashSet<usize>>>();

        let mut clicks = clicks;
        clicks.sort_by(|a, b| b.len().cmp(&a.len()));

        let voltge = tools::get_between(s,"{","}")
                     .split(",")
                     .map(|x| x.parse::<u64>().unwrap() )
                     .collect::<Vec<u64>>();

        World 
        { 
            code,
            buttons,
            voltge,
        }       
    }

    #[allow(dead_code)]
    fn print(&self)
    {
        println!();
        
        println!("code: {:b}",self.code);
        for (i,b) in self.buttons.iter().enumerate()
        {
            println!("btn[{}]: {:b}",i,b);
        }

        for v in &self.voltge
        {
            println!("voltge: {}",v);
        }
    }

    fn try_all(&mut self)->usize
    {
        (0..1usize<<self.buttons.len())
        .map(|i|
            {
                let mut val=0;
                for j in 0..self.buttons.len()
                {                                
                    if (i & (1<<j)) !=0
                    {
                        val ^= self.buttons[j];
                    }
                }

                if val==self.code { i.count_ones() as usize } else { usize::MAX }
            }
        )
        .min()
        .unwrap()
    }

    fn lp_solver(&mut self)-> usize
    {
        let n = self.voltge.len();

        let matrix: Vec<Vec<u64>> = self.buttons
                                   .iter()
                                   .map(|btn| 
                                        {
                                            let mut row = vec![0u64; n];

                                            for i in 0..n
                                            {
                                                if (btn & (1<<i)) !=0
                                                {
                                                    row[i] = 1;
                                                }
                                            }
                                            row
                                        }                
                                   )
                                   .collect();


        let constraints = self.voltge.len();

        let mut vars = variables!();
        let x: Vec<_> = (0..self.buttons.len())
                        .map(|_| vars.add(variable().integer().min(0).max(300)))
                        .collect();

        let mut problem = vars.minimise(x.iter().sum::<Expression>()).using(highs);

        for i in 0..constraints 
        {
            let exp: Expression = x.iter()
                                   .zip(matrix.iter())
                                   .map(|(&xi, row)| xi * row[i] as i32)
                                   .sum();

            problem = problem.with(exp.eq(self.voltge[i] as i32));
        }

        let solution = problem.solve().unwrap();
        
        x.iter()
         .map(|&id| solution.value(id).round() as usize)
         .sum()       
    }
}


pub fn part1(data:&[String])->usize
{
    data.iter()
        .map(|s| World::new(s).try_all() )
        .sum()
}

pub fn part2(data:&[String])->usize
{
    data.iter()
        .map(|s| World::new(s).lp_solver() )
        .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day10");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[allow(unused)]
fn get_test_data()->Vec<String>
{
    vec![
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string(),
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string(),
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string()
    ]   
}

#[test]
fn test1()
{
    let v = get_test_data();
    assert_eq!(part1(&v),7);
}

#[test]
fn test2()
{
    let v = get_test_data();
    assert_eq!(part2(&v),33);
}

#[test]
fn test3()
{    
    assert_eq!(part1(&vec!["[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()]),2);
}

#[test]
fn part2_test1()
{    
    assert_eq!(part2(&vec!["[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()]),10);
}

#[test]
fn part2_test2()
{    
    assert_eq!(part2(&vec!["[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string()]),12);
}

#[test]
fn part2_test3()
{    
    assert_eq!(part2(&vec!["[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string()]),11);
}
