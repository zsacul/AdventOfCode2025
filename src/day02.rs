fn invalid(s:&str)->i64
{
    if s.chars().next().unwrap()=='0' || s.len()%2==0 && s[..s.len()/2].repeat(2)==s
    {
        s.parse().unwrap()
    }
      else
    {
        0
    }
}

pub fn part1(data:&[String])->i64
{
    let line = data[0].split(',').collect::<Vec<&str>>();

    line.iter()
        .map(|s| 
            {
                let ss = s.split('-').collect::<Vec<&str>>();
                let a = ss[0].parse::<i64>().unwrap();
                let b = ss[1].parse::<i64>().unwrap();

                (a..=b).map(|v| invalid(&v.to_string()[..]))
                       .sum::<i64>()                                
            }
        ).sum()
}

fn invalid2(s:&str)->i64
{
    if (1..=s.len()/2).any(|size| s.len()%size==0 && s[..size].repeat(s.len()/size)==s)
    {
        s.parse().unwrap()
    }
      else 
    {
        0
    }
}

pub fn part2(data:&[String])->i64
{
    let data = data[0].split(',').collect::<Vec<&str>>();

    data.iter()
        .map(|s| 
            {
                let line = s.split('-').collect::<Vec<&str>>();
                let a = line[0].parse::<i64>().unwrap();
                let b = line[1].parse::<i64>().unwrap();

                (a..=b).map(|v| invalid2(v.to_string().as_str()))
                       .sum::<i64>()
                
            }
        ).sum::<i64>()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day2");
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
