use std::collections::HashMap;

fn valid(s:&str)->Option<i64>
{
  //  println!("valid {}",s);
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
    let mut code = 0;
    let mut hash = HashMap::new();

    for s in tab.clone()
    {
        hash.insert(s, true);
    }

    tab.iter()
       .map(|s| 
            {
                let ss = s.split('-').collect::<Vec<&str>>();
                let a = ss[0].parse::<i64>().unwrap();
                let b = ss[1].parse::<i64>().unwrap();

                let inva:i64 = (a..=b).map(|v| valid(&v.to_string()[..]).unwrap_or(0) as i64)
                                  .sum();
                println!("{} {}",s,inva);
                inva
            }
        ).sum()
}

fn valid2(s:&str)->Option<i64>
{
  //  println!("valid {}",s);
    if s.chars().next().unwrap()=='0' 
    {
        return Some(s.parse().unwrap());
    }
    
    if s.len()%2!=0
    {
        return None;
    }


    for s in 1..=s.len()/2
    {
        let mut rep="".to_string();
        while rep
        if s.chars().nth(s-1).unwrap() >= s.chars().nth(s).unwrap()
        {
            return None;
        }
    }
/*
    for i in 0..s.len()/2
    {
        if s.chars().nth(i).unwrap() != s.chars().nth(s.len()/2+i).unwrap() 
        {
            return None;
        }
    }
     */

    Some(s.parse().unwrap())
}


pub fn part2(data:&[String])->i64
{
    let tab = data[0].split(',').collect::<Vec<&str>>();
    let mut code = 0;
    let mut hash = HashMap::new();

    for s in tab.clone()
    {
        hash.insert(s, true);
    }

    tab.iter()
       .map(|s| 
            {
                let ss = s.split('-').collect::<Vec<&str>>();
                let a = ss[0].parse::<i64>().unwrap();
                let b = ss[1].parse::<i64>().unwrap();

                let inva:i64 = (a..=b).map(|v| valid(&v.to_string()[..]).unwrap_or(0) as i64)
                                  .sum();
                println!("{} {}",s,inva);
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

#[test]
fn test3()
{
    let v = vec![
    ];
    assert_eq!(part2(&v),10);
}
