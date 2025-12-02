pub fn part1(data:&[String])->usize
{
    let mut code = 50;

    data.iter()
        .filter(|s| 
            {
                let amount = s[1..].parse::<i32>().unwrap();
                let dir = s.chars().next().unwrap();

                if dir=='L' { code -= amount; } 
                       else { code += amount; }

                code%100==0
            }
        ).count()   
}

pub fn part2(data:&[String])->i32
{
    data.iter()
        .fold((50,0),|(code,res),s| 
            {
                let mut res= res;
                let mut code = code;
                let mut amount = s[1..].parse::<i32>().unwrap();
                let dir = s.chars().next().unwrap();

                let full_rot = amount/100;

                if full_rot>0 {
                    res += full_rot;
                    amount %= 100;
                }
                let was_zero = code==0;
                
                if dir=='L' { code -= amount; } 
                       else { code += amount; }

                if code==0 {
                    res+=1;
                }
                  else
                {
                    if code<0 {
                        code+=100;
                        if !was_zero { res+=1; }
                    }                        
                    if code>99 {
                        code-=100;
                        res+=1;                            
                    }                        
                }
                (code,res)
            }
        ).1
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
  ];
    assert_eq!(part1(&v),3);
}

#[test]
fn test2()
{
    let v = vec![
    ];
    assert_eq!(part2(&v),6);
}

