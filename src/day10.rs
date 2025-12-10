use std::collections::HashMap;
use super::tools;

#[derive(Debug)]
struct World
{
    des     : u64,
    buttons : Vec<u64>,
    volt    : Vec<u64>,
}

impl World
{
    fn to_bin(s:String)->u64
    {
        s.split(",")
         .fold(0, |acc,v| acc | 1<<v.parse::<u64>().unwrap() )
    }

    fn new(s:&String,part2:bool)->World 
    {
        let t = tools::get_between(s,"[","]");
        let t = t.replace('.', "0");
        let t = t.replace('#', "1");
        let n = t.len();

        //pares binary number
        let des:u64 = t.chars().rfold(0u64,|acc,c| acc*2 + (c as u64)-('0' as u64) );

        let butons = tools::get_between(s,"] "," {");

        let buttons:Vec<u64> = butons
            .split(" ")
            .map(|x| World::to_bin(tools::get_between(x, "(", ")") ))
            .collect();
        

        World 
        { 
            des     ,
            buttons : buttons,
            volt    : vec![],
        }       
    }

    #[allow(dead_code)]
    fn print(&self)
    {
        println!();
        
        println!("des: {:b}",self.des);
        for (i,b) in self.buttons.iter().enumerate()
        {
            println!("btn[{}]: {:b}",i,b);
        }


    }

    fn num_ones(&self,v:u64)->usize
    {
        let mut c = 0;
        let mut x = v;
        while x>0
        {
            if (x & 1) !=0 { c+=1; }
            x >>=1;
        }
        c
    }

    fn go(&mut self)->usize
    {
        let lim = 1<<self.buttons.len();
        let mut res = usize::MAX;

        for i in 0..lim
        {
            let mut val=0;
            for j in 0..self.buttons.len()
            {                                
                if (i & (1<<j)) !=0
                {
                    val ^= self.buttons[j];
                }
            }

            if val == self.des
            {
                let mm = self.num_ones(i as u64);
                if mm< res
                {
                    println!("found: {:b} with {} buttons",val,mm);
                }
                res = res.min(mm);
            }
        }
        res               
    }



    fn calc(&mut self)->usize
    {
        self.print();
        self.go()
    }

    fn calc2(&mut self)->usize
    {
        self.go()
    }

}


pub fn part1(data:&[String])->usize
{
    data.iter()
        .map(|s| World::new(s,false).calc() )
        .sum()
}

pub fn part2(data:&[String])->usize
{
    5555    
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
    assert_eq!(part2(&v),0);
}

#[test]
fn test3()
{    
    assert_eq!(part1(&vec!["[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()]),2);
}
