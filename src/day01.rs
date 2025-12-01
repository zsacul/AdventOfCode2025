pub fn part1(data:&[String])->usize
{
    let mut code = 50;

    data.iter()
        .filter(|s| 
            {
                let amount = s[1..].parse::<i32>().unwrap();               

                if s.chars().next().unwrap()=='L' {
                    code -= amount;                                       
                } else {
                    code += amount;
                }

                code%100==0
            }
        ).count()   
}

pub fn part2(data:&[String])->usize
{
    let mut res = 0;
    let mut code = 50;

    data.iter()
        .for_each(|s| 
            {
                let mut amount = s[1..].parse::<i128>().unwrap();
                let dir = s.chars().next().unwrap();

                let rot = amount/100;
                if rot>0 {
                    res += rot as usize;
                    amount = amount % 100;
                }
                
                if dir=='L' {
                    let mut was = code==0;
                    code -= amount;
                    
                    if code==0 {
                        res+=1;
                    }
                    else
                    {                        
                        while code<0 {
                            code+=100;

                            if !was { res+=1;}
                            was = false;
                        }                        
                    }
                } 
                  else           
                {
                    code += amount;

                    if code==0 
                    {
                        res+=1;
                        println!("{} at zero",s);
                    }
                      else 
                    {
                        while code>99 {
                            code-=100;
                            res+=1;                            
                        }                        
                    }
                }              
            }
        );
        
    res as usize
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day1");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "L68".to_string(),
        "L30".to_string(),
        "R48".to_string(),
        "L5".to_string(),
        "R60".to_string(),
        "L55".to_string(),
        "L1".to_string(),
        "L99".to_string(),
        "R14".to_string(),
        "L82".to_string(),
  ];
    assert_eq!(part1(&v),3);
}

#[test]
fn test2()
{
    let v = vec![
        "L68".to_string(),
        "L30".to_string(),
        "R48".to_string(),
        "L5".to_string(),
        "R60".to_string(),
        "L55".to_string(),
        "L1".to_string(),
        "L99".to_string(),
        "R14".to_string(),
        "L82".to_string(),
    ];
    assert_eq!(part2(&v),6);
}

#[test]
fn test3()
{
    let v = vec![
        "R1000".to_string(),
    ];
    assert_eq!(part2(&v),10);
}
