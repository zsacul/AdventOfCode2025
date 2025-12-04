use std::collections::HashMap;

struct Solve
{
    digits : Vec<i64>,
    memo   : HashMap<(usize,usize),i64>,
}

impl Solve {
    fn new(data:&str)->Self
    {
        let digits = data.chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<Vec<_>>();
        Solve {
            digits,
            memo: HashMap::new(),
        }
    }

    fn greatest(&mut self, n:usize, start:usize)->i64
    {
        if let Some(&val) = self.memo.get(&(n, start)) { return val; }

        match n 
        {
            0 => 0,
            1 => self.digits[start..].iter().max().cloned().unwrap_or(-1),
            _ => {
                 if n>self.digits.len()-start 
                 {
                    self.memo.insert((n, start), -1);
                    return -1;
                 }

                 let res:i64 =
                 (start..self.digits.len())
                            .map(|i| {
                                let v = self.greatest(n-1,i+1);
                                if v>=0 { self.digits[i]*10i64.pow((n-1) as u32) + v } else { 0 } 
                            })
                            .max()
                            .unwrap_or(-1);

                 self.memo.insert((n, start), res);
                 res
            },
        }
    }
}

fn part1(data:&[String])->i64
{
    data.iter()
        .map(|s| Solve::new(s).greatest(2, 0) )
        .sum::<i64>()
}

fn part2(data:&[String])->i64
{
    data.iter()
        .map(|s| Solve::new(s).greatest(12, 0) )
        .sum::<i64>()
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

