use std::collections::HashMap;
use std::usize;
use super::tools;        
use super::vec2::Vec2;

#[derive(Debug)]
struct Data {
      pairs: Vec<Vec2>,
      map: HashMap<Vec2,char>,
      xb : HashMap<i64,i64>,
      yb : HashMap<i64,i64>,
}

impl Data {
    fn new(input: &[String]) -> Self {
        
        let mut xx = vec![];
        let mut yy = vec![];
        
        let _pairs = input.iter()
            .map(|line| {
                {
                    let s = tools::split_to_usize(line,",");                
                    let px = s[0] as i64;
                    let py = s[1] as i64;
                    xx.push(px);
                    yy.push(py);
                    Vec2::new(px,py)
                }
            })
            .collect::<Vec<_>>();

        xx.sort_unstable();
        yy.sort_unstable();
        xx.dedup();
        yy.dedup();

        let mut x = HashMap::new();
        let mut y = HashMap::new();
        let mut xb = HashMap::new();
        let mut yb = HashMap::new();

        for ix in 0..xx.len() {
            x.insert(xx[ix], ix as i64);

            
                xb.insert(ix as i64, xx[ix]);            
        }
        for iy in 0..yy.len() {
            y.insert(yy[iy], iy as i64);
            yb.insert(iy as i64, yy[iy]);
        }


        let pairs = input.iter()
            .map(|line| {
                {
                    let s = tools::split_to_usize(line,",");                
                    let px = s[0] as i64;
                    let py = s[1] as i64;
                    Vec2::new(*x.get(&px).unwrap(), *y.get(&py).unwrap())
                }
            })
            .collect();        
        
        Data {
            pairs,
            map: HashMap::new(),
            xb,
            yb,
        }
    }

    fn println_map(&self)
    {
        let x0 = self.map.keys().map(|v| v.x).min().unwrap();
        let y0 = self.map.keys().map(|v| v.y).min().unwrap();
        let x1 = self.map.keys().map(|v| v.x).max().unwrap();
        let y1 = self.map.keys().map(|v| v.y).max().unwrap();

        for y in y0..=y1
        {
            for x in x0..=x1
            {
                let v = self.map.get(&Vec2::new(x,y)).unwrap_or(&'.');
                print!("{}",v);
            }
            println!();
        }
        println!();
    }

    fn area(&self,a:&Vec2,b:&Vec2)->usize
    {
        let ax = self.xb.get(&a.x).unwrap();
        let ay = self.yb.get(&a.y).unwrap();
        let bx = self.xb.get(&b.x).unwrap();
        let by = self.yb.get(&b.y).unwrap();
        (((bx - ax).abs()+1) * ((by - ay).abs()+1)) as usize
    }

    fn count1(&self)->usize
    {
        let mut res = 0;
        for i in 0..self.pairs.len()
        {
            for j in i+1..self.pairs.len()
            {
                let a = &self.pairs[i];
                let b = &self.pairs[j];

                let area = self.area(a,b);
                res = res.max(area);
            }
        }
        res as usize
   
    }

    fn get(&self,x:i64,y:i64)->char
    {
        self.map.get(&Vec2::new(x,y)).cloned().unwrap_or('.')
    }

    fn is_inside(&self,a:&Vec2,b:&Vec2)->bool
    {
        let x1 = a.x.min(b.x);
        let x2 = a.x.max(b.x);
        let y1 = a.y.min(b.y);
        let y2 = a.y.max(b.y);

        for y in y1..=y2
        {
            for x in x1..=x2
            {
                if self.get(x,y)=='.'
                {
                    return false;
                }
            }
        }
        true       
    }


    fn draw_line(&mut self,a:&Vec2,b:&Vec2,c:char)
    {
        let x1 = a.x.min(b.x);
        let x2 = a.x.max(b.x);
        let y1 = a.y.min(b.y);
        let y2 = a.y.max(b.y);

        if x1!=x2 && y1!=y2
        {
            return;
        }

        if x1==x2
        {
            for y in y1..=y2
            {
                self.map.insert( Vec2::new(x1,y), c);
            }                    
        }
          else 
        {
            for x in x1..=x2
            {
                self.map.insert( Vec2::new(x,y1), c);
            }
        }
    }

    fn flood_fill(&mut self,x:i64,y:i64,c:char)
    {
        let mut stack = vec![Vec2::new(x,y)];        

        while !stack.is_empty()
        {
            let p= stack.pop().unwrap();
            if self.get(p.x,p.y)!='.'
            {
                continue;
            }
            self.map.insert(Vec2::new(p.x,p.y), c);
            stack.push( Vec2::new(p.x+1,p.y) );
            stack.push( Vec2::new(p.x-1,p.y) );
            stack.push( Vec2::new(p.x,p.y+1) );
            stack.push( Vec2::new(p.x,p.y-1) );
        }

    }
   
    fn count2(&mut self)->usize
    {
        self.pairs.clone().windows(2)
            .map(|w| (w[0],w[1]))
            .for_each(|(a,b)| {
                self.draw_line(&a.clone(),&b.clone(),'#');
            });

        self.flood_fill(124,44,'#');

        let mut res = 0;
        for i in 0..self.pairs.len()
        {
            for j in 0..self.pairs.len()
            {
                if i==j
                {
                    continue;
                }
                let a = &self.pairs[i];
                let b = &self.pairs[j];               
                let area = self.area(a, b);
                
                if area>res
                {
                    if self.is_inside(a,b)
                    {
                        res = res.max(area);
                    }
                    //print!("{} {:?} {:?}\n",area,a,b);
                }                               
            }
        }

        self.println_map(); 
        res as usize
    }

}

pub fn part1(data:&[String])->usize
{
    Data::new(data).count1()
}

pub fn part2(data:&[String])->usize
{
    Data::new(data).count2()
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day9");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "7,1".to_string(),
        "11,1".to_string(),
        "11,7".to_string(),
        "9,7".to_string(),
        "9,5".to_string(),
        "2,5".to_string(),
        "2,3".to_string(),
        "7,3".to_string(),
    ];
    assert_eq!(part1(&v),50);
}

#[test]
fn test2()
{
    let v = vec![
        "7,1".to_string(),
        "11,1".to_string(),
        "11,7".to_string(),
        "9,7".to_string(),
        "9,5".to_string(),
        "2,5".to_string(),
        "2,3".to_string(),
        "7,3".to_string(),
    ];
    assert_eq!(part2(&v),24);
}
