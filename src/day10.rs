use std::{collections::HashSet};
use std::{collections::HashMap};
use std::error::Error;

//use good_lp::{variables};
//use good_lp::solvers::Highs;

use super::tools;

#[derive(Debug)]
struct World
{
    des     : u64,
    buttons : Vec<u64>,
    volt    : Vec<u64>,
    clicks  : Vec<HashSet<usize>>,
    best    : i16,
    state   : Vec<u64>,
    memo    : HashMap<Vec<i16>,  i16>,
    sum     : usize,
    rand    : rand::rngs::ThreadRng,
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

        let mut clicks = clicks;
        clicks.sort_by(|a, b| b.len().cmp(&a.len()));


        let volt = tools::get_between(s,"{","}")
                            .split(",")
                            .map(|x| x.parse::<u64>().unwrap() )
                            .collect::<Vec<u64>>();
                        let n = buttons.len();

        let sum = volt.iter().sum::<u64>() as usize;

        World 
        { 
            des,
            buttons,
            clicks,
            volt,
            best  : 30000,
            state : vec![0; n],
            sum,
            memo  : HashMap::new(),
            rand  : rand::thread_rng(),
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
        let mut res = 30000;

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
/*
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

    fn click(&self,button:usize,left:&mut Vec<i16>)->i16
    {
        //let maxl = 1;//self.clicks[button]
                            //.iter()
                            //.map(|&c| left[c] )
                            //.max()
                            //.unwrap_or(0)
                            //.max(1);

        for c in &self.clicks[button]
        {
            left[*c] -= 1;
        }
        1
    }

    fn unclick(&self,button:usize,cc:i16,left:&mut Vec<i16>)
    {
        for c in &self.clicks[button]
        {
            left[*c] += cc;
        }
    }   
 
    fn newgo(&mut self,clicks:i16,mut left:Vec<i16>)->i16
    {

        //let v = self.voltage(&self.state);

        if clicks > self.best
        {
            return 30000;
        }

        if let Some(&m) = self.memo.get(&left)
        {
            return m;
        }
        
        if left.iter().all(|&n| n==0)
        {                  
            self.memo.insert(left.clone(), 0);      
            return 0;
        }

        if left.iter().any(|&n| n<0)
        {                  
            self.memo.insert(left.clone(), 30000);      
            return 30000;
        }
        
        let store = left.clone();
        let mut res = 30000;

        let mut order = (0..self.buttons.len())
            .collect::<Vec<usize>>();
        order.shuffle(  &mut self.rand);

        for ii in &order
        {
            let i = *ii;
            let cc = self.click(i,&mut left);
            let run = self.newgo(clicks+cc,left.clone());
            self.unclick(i,cc,&mut left);
            
            if run+1< res && run!=30000
            {
                res = run+1;

                if clicks==0
                {
                    if res<self.best
                    {
                        println!("left {:?} clicking {} buttons {} gives {}",left.clone(),clicks,i,run);
                        self.best = res;
                    }
                }
            }
        }

        self.memo.insert(store, res);
        res
        
    }
*/
/*
    fn fl_solver2(&self)->usize
    {
        let mut vars = variables!();
        let a = vars.add(variable().binary());
        let b = vars.add(variable().binary());

        let solution = vars.maximise(a + 2 * b)
        .using(HighsSolver::default())  // <-- This replaces .using(default_solver)
        .with((a + b) <= 1)
        .with(a - b == 0)
        .solve()
        .unwrap();  // Or handle the Result properly

        println!("a = {}, b = {}", solution.value(a), solution.value(b));
        0
    }
*/
    fn lp_solver(&mut self)-> Result<usize, Box<dyn Error>>
    {
        let n = self.volt.len();

   //assert_eq!(part2(&vec!["[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string()]),11);
   //(0,1,2,3,4) 
   //(0,3,4) 
   //(0,1,2,4,5) 
   //(1,2) 
   //{10,11,11,5,10,5}
/*
        let m = vec![vec![0;n]; self.buttons.len()];
        
        variables! {
            vars:
            0 <= x0 (integer) <= 290;
            0 <= x1 (integer) <= 290;
            0 <= x2 (integer) <= 290;
            0 <= x3 (integer) <= 290;
            0 <= x4 (integer) <= 290;
        }; // variables can also be added dynamically with ProblemVariables::add

        let solution = 
        vars.minimise(x0+x1+x2+x3+x4)
            //.using(default_solver) // IBM's coin_cbc by default
//            .using(HighsSolver::default())
            .with(x0*m[0][0]+x1*m[1][0]+x2*m[2][0]+x3*m[3][0]+x4*m[4][0]==self.volt[0] as i32)
            .with(x0*m[0][1]+x1*m[1][1]+x2*m[2][1]+x3*m[3][1]+x4*m[4][1]==self.volt[1] as i32)
            .with(x0*m[0][2]+x1*m[1][2]+x2*m[2][2]+x3*m[3][2]+x4*m[4][2]==self.volt[2] as i32)
            .with(x0*m[0][3]+x1*m[1][3]+x2*m[2][3]+x3*m[3][3]+x4*m[4][3]==self.volt[3] as i32)
            .with(x0*m[0][4]+x1*m[1][4]+x2*m[2][4]+x3*m[3][4]+x4*m[4][4]==self.volt[4] as i32)
            .solve()
            .unwrap()          ;
        
        let sol = 
        vec![
            solution.value(x0) as u64,
            solution.value(x1) as u64,
            solution.value(x2) as u64,
            solution.value(x3) as u64,
            solution.value(x4) as u64,
        ];

        println!("x0={}", sol[0]);
        println!("x1={}", sol[1]);
        println!("x2={}", sol[2]);
        println!("x3={}", sol[3]);        
        println!("x4={}", sol[4]);        
        let s = sol.iter().sum::<u64>() as usize;

        Ok(s)
        */
        Ok(0)
    }

    fn calc(&mut self)->usize
    {
        self.print();
        self.go()
    }

    fn calc2(&mut self)->usize
    {
       // self.print();
        self.best = self.sum as i16;
        self.state = vec![0; self.buttons.len()];
        //let left =  self.volt.iter().map(|n| *n as i16).collect::<Vec<i16>>();
        //self.newgo(0,  left) as usize
        self.lp_solver().unwrap() as usize
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
 //   println!("part1: {}",part1(data));
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
