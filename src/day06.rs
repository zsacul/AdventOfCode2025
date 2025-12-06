#[derive(Debug,Clone)]
struct Equation
{
    nums     : Vec<String>,
    operator : char,
}

impl Equation
{
    fn new()->Self
    {
        Equation::from(vec![], '?')
    }

    fn from(nums:Vec<String>,operator:char)->Self
    {
        Equation 
        {
            nums,
            operator,
        }
    }

    #[allow(unused)]
    fn println(&self)
    {
        println!("Equation: {:?} operator={}", self.nums, self.operator);
    }

    fn calculate(&self)->usize
    {
        self.nums.iter()
                 .fold(if self.operator == '+' { 0 } else { 1 }, |acc, s| 
                    {
                        let num = s.parse::<usize>().unwrap();

                        match self.operator
                        {
                            '+' => acc + num,
                            '*' => acc * num,
                            _   => panic!("Unknown operator"),
                        }
                    })        
    }

}

struct Solve {
    equations: Vec<Equation>,
}

impl Solve {
    fn new(data:&[String])->Self
    {
        let operators = Solve::operators(data);
        let mut equ = vec![Equation::new(); operators.len()];

        for row in data.iter().take(data.len()-1)
        {
            let parts = row.split_whitespace().collect::<Vec<&str>>();

            for i in 0..parts.len()
            {
                equ[i].nums.push(parts[i].to_string());
                equ[i].operator = operators[i];
            }
        }

        Solve 
        {
            equations: equ,
        }
    }

    fn new2(data:&[String])->Self
    {
        let operators = Solve::operators(data);
        let data2 = Solve::transpose_matrix(data);        

        let sections: Vec<&[String]> = data2[..data2.len()-1].split(|line| line.is_empty()).collect();

        let equ = sections.iter().enumerate().map(|(id,&sec)| 
        {
            Equation::from(
                sec.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
                operators[operators.len()-1-id]
            )
        })
        .collect::<Vec<Equation>>();

        Solve 
        {
            equations: equ,
        }
    }    

    fn operators(data:&[String])->Vec<char>
    {
        data.last()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .chars()
            .collect::<Vec<char>>()
    }

    fn transpose_matrix(d:&[String])->Vec<String>
    {
        let mut result : Vec<String> = vec![];

        let rows = d.len();
        let cols = d[0].len();

        for col_id in (0..cols).rev()
        {   
            let mut new_row = String::new();

            for row in d.iter().take(rows-1)
            {
                new_row.push(row.chars().nth(col_id).unwrap());
            }

            result.push(new_row.trim().to_string());
        }

        result.push(d[rows-1].clone());

        result
    }

    #[allow(unused)]
    fn println(&self)   
    {
        for eq in &self.equations
        {
            eq.println();
        }
    }

    fn calculate(&self)->usize
    {
        self.equations.iter()
            .map(|eq| eq.calculate() )
            .sum()
    }
}

fn part1(data:&[String])->usize
{
    Solve::new(data).calculate()
}

fn part2(data:&[String])->usize
{
    Solve::new2(data).calculate()   
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day6");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
    "123 328  51 64 ".to_string(),
    " 45 64  387 23 ".to_string(),
    "  6 98  215 314".to_string(),
    "*   +   *   +          ".to_string(),
    ];
    assert_eq!(part1(&v),4277556);
}

#[test]
fn test2()
{
    let v = vec![
    "123 328  51 64 ".to_string(),
    " 45 64  387 23 ".to_string(),
    "  6 98  215 314".to_string(),
    "*   +   *   +          ".to_string(),
    ];
    assert_eq!(part2(&v),3263827);
}