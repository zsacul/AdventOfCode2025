use std::{collections::HashSet};
use std::{collections::HashMap};
use super::tools;

#[derive(Debug)]
struct World
{
    des     : u64,
    buttons : Vec<u64>,
    volt    : Vec<u64>,
    clicks  : Vec<HashSet<usize>>,
    best    : usize,
    state   : Vec<u64>,
    n:usize,
    memo: HashMap<Vec<u64>, (bool, usize)>,
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

        //pares binary number
        let des:u64 = t.chars().rfold(0u64,|acc,c| acc*2 + (c as u64)-('0' as u64) );

        let butons = tools::get_between(s,"] "," {");

        let buttons:Vec<u64> = butons
            .split(" ")
            .map(|x| World::to_bin(tools::get_between(x, "(", ")") ))
            .collect();

        let clicks =  butons
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


        let volt = tools::get_between(s,"{","}")
                            .split(",")
                            .map(|x| x.parse::<u64>().unwrap() )
                            .collect::<Vec<u64>>();
                        let n = buttons.len();

        World 
        { 
            des,
            buttons,
            clicks,
            volt,
            best    : usize::MAX,
            state   : vec![0; n],
            n,
            memo    : HashMap::new(),
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

        for v in &self.volt
        {
            println!("volt: {}",v);
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

        for i in 0..lim as usize 
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

    fn voltage(&self,state:&[u64])->Vec<u64>
    {
        let mut res = vec![0; self.volt.len()];

        for (i,n) in state.iter().enumerate()
        {
            for c in &self.clicks[i]
            {
                res[*c] += n;
            }
        }
        res
    }    

    fn get_clicks(&self,state:&[u64])->usize
    {
        state.iter()        
        .sum::<u64>() as usize

//        state.iter()
//            .enumerate()
//            .map(|(i,&b)| b as usize * self.clicks[i].len() )
//            .sum()
    }

    fn go2(&mut self)->(bool,usize)
    {
        if let Some(v) = self.memo.get(&self.state)
        {
            return *v;
        }   
        let v = self.voltage(&self.state);
        let mut res = (false,usize::MAX);
        
        let clicks = self.get_clicks(&self.state);

        if v==self.volt
        {                        
            self.memo.insert(self.state.clone(), (true,self.get_clicks(&self.state)));
            return (true,self.get_clicks(&self.state));
        }

        if clicks > self.best
        {
            self.memo.insert(self.state.clone(), (false,res.1));
            return (false,res.1);
        }

        for i in 0..v.len()
        {
            if v[i] > self.volt[i]
            {
                self.memo.insert(self.state.clone(), (false,res.1));
                return (false,res.1);
            }
        }

        for i in 0..self.buttons.len()
        {
            self.state[i] += 1;
            let run = self.go2();
            self.state[i] -= 1;
            
            if run.0
            {
                let nv = run.1;
                
                if nv<=self.best
                {
                   // println!("found voltage with {} clicks",nv);
                    self.best = nv;
                    res = (true,nv);
                }
            }
        }

        self.memo.insert(self.state.clone(), res);
        res
    }

    fn calc(&mut self)->usize
    {
        self.print();
        self.go()
    }

    fn calc2(&mut self)->usize
    {
        self.print();
        self.best = usize::MAX;
        self.state = vec![0; self.buttons.len()];
        self.go2().1
    }

}


pub fn part1(data:&[String])->usize
{
    data.iter()
        .map(|s| World::new(s).calc() )
        .sum()
}

pub fn part2(data:&[String])->usize
{
    data.iter()
        .map(|s| World::new(s).calc2() )
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
fn test4()
{    
    assert_eq!(part2(&vec!["[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()]),10);
}
