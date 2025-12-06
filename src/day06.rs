use num::BigInt;

#[derive(Debug,Clone)]
struct Equation
{
    nums : Vec<String>,
    operator  : char,
}

impl Equation
{
    fn new()->Self
    {
        Equation {
            nums: vec![],
            operator: '?',
        }
    }

    fn println(&self)
    {
        println!("Equation: nums={:?}, operator={}", self.nums, self.operator);
    }

    fn calculate(&self)->BigInt
    {
        let mut result = if self.operator == '+' { BigInt::from(0) } else { BigInt::from(1) };

        for num in &self.nums
        {
            println!("num: {}", num);
            let num = BigInt::parse_bytes(num.as_bytes(), 10).unwrap();

            match self.operator
            {
                '+' => result += num,
                '*' => result *= num,
                _   => {},
            }
        }

        result
    }

}

struct Solve {
    equations: Vec<Equation>,
}

impl Solve {
    fn new(data:&[String])->Self
    {
        let operators = data[data.len()-1].split_whitespace().collect::<Vec<&str>>().join("").chars().collect::<Vec<char>>();
        let mut equ = vec![Equation::new(); operators.len()];

        for i in 0..data.len()-1
        {
            let parts = data[i].split_whitespace().collect::<Vec<&str>>();

            for i in 0..parts.len()
            {
                equ[i].nums.push(parts[i].to_string());
                equ[i].operator = operators[i];
            }
        }

        Solve {
            equations: equ,
        }
    }

    fn new2(data:&[String])->Self
    {
        let operators = data[data.len()-1].split_whitespace().collect::<Vec<&str>>().join("").chars().collect::<Vec<char>>();
        
        let n = operators.len();

        let data2 = Solve::transpose_matrix(data);
        let data2 = data2[..data2.len()-1].to_vec();
        let sections: Vec<&[String]> = data2.split(|line| line.is_empty()).collect();
        let mut equ = vec![Equation::new(); sections.len()];

        for (id,&sec) in sections.iter().enumerate()
        {            
            equ[id].nums = sec.iter().map(|s| s.to_string()).collect::<Vec<String>>();            
            equ[id].operator = operators[operators.len()-1-id];
        }

        Solve {
            equations: equ,
        }
    }    

    fn transpose_matrix(d:&[String])->Vec<String>
    {
        let mut result:Vec<String> = vec![];

        let row_count = d.len();
        let col_count = d[0].len();

        for col in (0..col_count).rev()
        {
            let mut new_row = String::new();

            for row in 0..row_count-1
            {
                let parts = d[row].chars().nth(col).unwrap();
                new_row.push(parts);
            }

            result.push(new_row.trim().to_string());
        }

        result.push(d[row_count-1].clone());

        result
    }

    fn println(&self)   
    {
        for eq in &self.equations
        {
            eq.println();
        }
    }

    fn calculate(&self)->String
    {
        let mut result = BigInt::from(0);

        for eq in &self.equations
        {
            let num = eq.calculate();
            println!("equation result: {}", num);
            result+= num;
        }

        result.to_string()
    }
}

fn part1(data:&[String])->String
{
    Solve::new(data).calculate()
}

fn part2(data:&[String])->String
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
    assert_eq!(part1(&v),"4277556".to_string());
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
    assert_eq!(part2(&v),"3263827".to_string());
}
