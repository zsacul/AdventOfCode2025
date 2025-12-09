use std::collections::HashMap;
use super::tools;        
use super::vec2::Vec2;

#[derive(Debug)]
struct Data {
      pairs  : Vec<Vec2>,
      map    : HashMap<Vec2,char>,
      x_back : HashMap<i64,i64>,
      y_back : HashMap<i64,i64>,
      min    : Vec2,
      max    : Vec2,
}

impl Data {
    fn new(input: &[String]) -> Self {
        
        let mut xx = vec![];
        let mut yy = vec![];
        
        let pairs = input.iter()
                         .map(|line| {
                             {
                                 let s = tools::split_to_i64(line,",");                
                                 xx.push(s[0]);
                                 yy.push(s[1]);
                                 Vec2::new(s[0],s[1])
                             }
                         })
                         .collect::<Vec<_>>();

        xx.sort_unstable();
        yy.sort_unstable();
        xx.dedup();
        yy.dedup();

        let mut x_map  = HashMap::new();
        let mut y_map  = HashMap::new();
        let mut x_back = HashMap::new();
        let mut y_back = HashMap::new();

        for (x_id, &x) in xx.iter().enumerate()
        {
            let nx = (x_id*2+1) as i64;
             x_map.insert(x ,nx);
            x_back.insert(nx,x );
        }
        
        for (y_id, &y) in yy.iter().enumerate()
        {
            let ny = (y_id*2+1) as i64;
             y_map.insert(y ,ny);
            y_back.insert(ny,y );
        }

        let pairs = pairs.iter()
                         .map(|p| Vec2::new(*x_map.get(&p.x).unwrap(), *y_map.get(&p.y).unwrap()))
                         .collect::<Vec<Vec2>>();      

        let rx0 = pairs.iter().map(|v| v.x).min().unwrap();
        let ry0 = pairs.iter().map(|v| v.y).min().unwrap();
        let rx1 = pairs.iter().map(|v| v.x).max().unwrap();
        let ry1 = pairs.iter().map(|v| v.y).max().unwrap();
        
        Data {
            pairs,
            map: HashMap::new(),
            x_back,
            y_back,
            min: Vec2::new(rx0, ry0),
            max: Vec2::new(rx1, ry1),
        }
    }

    #[allow(unused)]
    fn println_map(&self)
    {
        for y in self.min.y..=self.max.y
        {
            for x in self.min.x..=self.max.x
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
        let ax = self.x_back.get(&a.x).unwrap();
        let ay = self.y_back.get(&a.y).unwrap();
        let bx = self.x_back.get(&b.x).unwrap();
        let by = self.y_back.get(&b.y).unwrap();
        (((bx - ax).abs()+1) * ((by - ay).abs()+1)) as usize
    }

    fn count1(&self)->usize
    {
        (0..self.pairs.len()).map(|i| 
                                (0..self.pairs.len()).map(|j| self.area(&self.pairs[i], &self.pairs[j])).max().unwrap()                
                            ).max().unwrap()   
    }

    fn get(&self,x:i64,y:i64)->char
    {
        *self.map.get(&Vec2::new(x,y)).unwrap_or(&'.')
    }

    fn all_inside(&self,a:&Vec2,b:&Vec2)->bool
    {
        let x1 = a.x.min(b.x);
        let x2 = a.x.max(b.x);
        let y1 = a.y.min(b.y);
        let y2 = a.y.max(b.y);
        
        (y1..=y2).all(|y|
        (x1..=x2).all(|x| self.get(x,y)!='.'))
    }

    fn draw_line(&mut self,a:&Vec2,b:&Vec2,c:char)
    {
        let x1 = a.x.min(b.x);
        let x2 = a.x.max(b.x);
        let y1 = a.y.min(b.y);
        let y2 = a.y.max(b.y);

        if x1!=x2 && y1!=y2 { return; }

        if x1==x2
        {
            for y in y1..=y2 { self.map.insert( Vec2::new(x1,y), c); }                    
        }
          else 
        {
            for x in x1..=x2 { self.map.insert( Vec2::new(x,y1), c); }
        }
    }

    fn flood_fill(&mut self,x:i64,y:i64,c:char)
    {
        let mut stack = vec![Vec2::new(x,y)];        

        while let Some(p) = stack.pop()
        {
            if self.get(p.x,p.y)!='.' { continue; }
            self.map.insert(Vec2::new(p.x,p.y), c);

            stack.push( Vec2::new(p.x+1,p.y  ) );
            stack.push( Vec2::new(p.x-1,p.y  ) );
            stack.push( Vec2::new(p.x  ,p.y+1) );
            stack.push( Vec2::new(p.x  ,p.y-1) );
        }
    }
   
    fn count2(&mut self)->usize
    {
        let n = self.pairs.len();

        self.pairs
            .clone()
            .windows(2)            
            .for_each(|w| {
                self.draw_line(&w[0],&w[1],'#');
            });

        
        self.draw_line(&self.pairs[n-1].clone(),&self.pairs[0].clone(),'#');

        self.flood_fill((self.min.x+self.max.x)/2,(self.min.y+self.max.y)/2 + (self.max.y - self.min.y)/7,'#');

        let mut res = 0;
        for i in 0..n
        {
            for j in i+1..n
            {
                let a = &self.pairs[i];
                let b = &self.pairs[j];               
                let area = self.area(a, b);
                
                if area>res && self.all_inside(a,b)
                {
                    res = res.max(area);
                }                               
            }
        }

        res
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

#[allow(unused)]
fn get_test_data()->Vec<String>
{
    vec![
        "7,1".to_string(),
        "11,1".to_string(),
        "11,7".to_string(),
        "9,7".to_string(),
        "9,5".to_string(),
        "2,5".to_string(),
        "2,3".to_string(),
        "7,3".to_string(),
    ] 
}

#[test]
fn test1()
{
    assert_eq!(part1(&get_test_data()),50);
}

#[test]
fn test2()
{
    assert_eq!(part2(&get_test_data()),24);
}
