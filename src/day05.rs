use super::tools;

#[derive(Debug)]
struct Data {
      pairs: Vec<(usize,usize)>,
    numbers: Vec<usize>,
        tab: Vec<usize>,
}

impl Data {
    fn new(input: &[String]) -> Self {
        let sections: Vec<&[String]> = input.split(|line| line.is_empty()).collect();
        
        let pairs = sections[0].iter()
            .map(|line| {
                let nums = tools::split_to_usize(line, "-");
                (nums[0].min(nums[1]), nums[0].max(nums[1]))
            })
            .collect();
        
        let numbers: Vec<usize> = sections[1].iter()
            .map(|line| line.parse().unwrap())
            .collect();

        Data {
            pairs,
            numbers,
            tab: vec![],
        }
    }

    fn ok1_line(&self, n:usize) -> bool
    {
        for &(a,b) in &self.pairs
        {
            if a<=n && b>=n 
            {
                return true;
            }            
        }
        false
    }



    fn ok1(&self) -> usize
    {
        self.numbers.iter()
            .filter(|&&l| self.ok1_line(l))
            .count()
    }


    fn prepare(&mut self)
    {
        let mut t = vec![];
        for &(a,b) in self.pairs.iter()
        {
            t.push(a);
            t.push(b);
        }
        t.sort();
        t.dedup();
        self.tab = t;
    }
  
    fn ok2(&self) -> usize
    {
        let mut res =0;
        let mut s = self.tab[0]-1;

        println!("tab: {:#?}", self.tab);

        for &i in self.tab.iter()
        {
            let delta = if self.ok1_line(s+1) 
            {
                i-s
            }
                else 
            {
                1usize
            };
            
            res += delta;
            s=i;
        }
        res
    }
}

pub fn part1(data:&[String])->usize
{
    Data::new(data).ok1()
}

pub fn part2(data:&[String])->usize
{    
    let mut data = Data::new(data);
    data.prepare();
    data.ok2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day5");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "3-5".to_string(),
        "10-14".to_string(),
        "16-20".to_string(),
        "12-18".to_string(),
        "".to_string(),
        "1".to_string(),
        "5".to_string(),
        "8".to_string(),
        "11".to_string(),
        "17".to_string(),
        "32".to_string(),
    ];
    assert_eq!(part1(&v),3);
}

#[test]
fn test2()
{
    let v = vec![
        "3-5".to_string(),
        "10-14".to_string(),
        "16-20".to_string(),
        "12-18".to_string(),
        "".to_string(),
        "1".to_string(),
        "5".to_string(),
        "8".to_string(),
        "11".to_string(),
        "17".to_string(),
        "32".to_string(),        
    ];
    assert_eq!(part2(&v),14);
}
