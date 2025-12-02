fn invalid(s:&str)->Option<i64>
{
    if s.chars().next().unwrap()=='0' 
    {
        return Some(s.parse().unwrap());
    }
    
    if s.len()%2!=0
    {
        return None;
    }

    for i in 0..s.len()/2
    {
        if s.chars().nth(i).unwrap() != s.chars().nth(s.len()/2+i).unwrap() 
        {
            return None;
        }
    }

    Some(s.parse().unwrap())
}

pub fn part1(data:&[String])->i64
{
    let tab = data[0].split(',').collect::<Vec<&str>>();

    tab.iter()
       .map(|s| 
            {
                let ss = s.split('-').collect::<Vec<&str>>();
                let a = ss[0].parse::<i64>().unwrap();
                let b = ss[1].parse::<i64>().unwrap();

                let inva:i64 = (a..=b).map(|v| invalid(&v.to_string()[..]).unwrap_or(0) as i64)
                                  .sum();
                println!("{} {}",s,inva);
                inva
            }
        ).sum()
}

fn invalid2(s:&str)->Option<i64>
{
    for si in 1..=s.len()/2
    {      
        if s[..si].repeat(s.len()/si)==s
        {
            return Some(s.parse().unwrap());            
        }
    }

    return None;
}

pub fn part2(data:&[String])->i64
{
    let tab = data[0].split(',').collect::<Vec<&str>>();

    tab.iter()
       .map(|s| 
            {
                let ss = s.split('-').collect::<Vec<&str>>();
                let a = ss[0].parse::<i64>().unwrap();
                let b = ss[1].parse::<i64>().unwrap();

                let inva:i64 = (a..=b).map(|v| invalid2(&v.to_string()[..]).unwrap_or(0) as i64)
                                  .sum();

                inva
            }
        ).sum()
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
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string()
  ];
    assert_eq!(part1(&v),1227775554);
}

#[test]
fn test2()
{
    let v = vec![
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string()
    ];
    assert_eq!(part2(&v),4174379265);
}
