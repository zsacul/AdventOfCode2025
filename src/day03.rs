use std::collections::HashMap;

struct Solve
{
    digits: Vec<i64>,
    memo: HashMap<(usize,usize),i64>,
}

impl Solve {
    fn new(data:&str)->Self
    {
        let digits = data.chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<Vec<i64>>();
        Solve {
            digits,
            memo: HashMap::new(),
        }
    }

    fn greatest(&mut self, n:usize, start:usize)->i64
    {
        if let Some(&val) = self.memo.get(&(n, start)) {
            return val;
        }

        if start>self.digits.len()-n {
            self.memo.insert((n, start), -1);
            return -1;
        }
        if n==0  {
            self.memo.insert((n, start), 0);
            return 0;
        }
        if n==1 {
            self.memo.insert((n, start), self.digits[start..].iter().max().cloned().unwrap_or(-1));
            return self.digits[start..].iter().max().cloned().unwrap_or(-1)
        }

        let mut res:i64 =0;

        for i in start..self.digits.len()
        {
            let d = self.digits[i];
            let v = self.greatest(n-1,i+1);

            if v>=0
            {
                res = res.max(d*10i64.pow((n-1) as u32)+v);
            }
        }
        self.memo.insert((n, start), res);
        res
    }
}

fn part1(data:&[String])->i64
{
        data.iter()
        .map(|s| 
            {
                Solve::new(s).greatest(2, 0)
            }
        ).sum::<i64>()
}

fn part2(data:&[String])->i64
{
    data.iter()
        .map(|s| 
            {
                Solve::new(s).greatest(12, 0)
            }
        ).sum::<i64>()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day3");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
"987654321111111".to_string(),
"811111111111119".to_string(),
"234234234234278".to_string(),
"818181911112111".to_string(),
    ];
    assert_eq!(part1(&v),357);
}

#[test]
fn test2()
{
    let v = vec![
"987654321111111".to_string(),
"811111111111119".to_string(),
"234234234234278".to_string(),
"818181911112111".to_string(),
    ];
    assert_eq!(part2(&v),3121910778619);
}

