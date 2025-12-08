use std::collections::HashMap;
use super::vec3::Vec3;

#[derive(Eq, PartialEq, Debug, Clone,Hash)]
struct Voxel
{
    pos :  Vec3,
    id  : usize,
}

impl Voxel 
{
    #[allow(unused)]
    fn new(x:i64,y:i64,z:i64,id:usize)->Self
    {
        Self 
        {
            pos: Vec3::new(x, y, z),
            id
        }
    }

    fn from_str(v:&str,id:usize)->Self
    {        
        let tab : Vec<&str> = v.split(",").collect();
        Self::new(
            tab[0].trim().parse::<i64>().unwrap(),
            tab[1].trim().parse::<i64>().unwrap(),
            tab[2].trim().parse::<i64>().unwrap(),
            id
        )
    }
}

struct Space
{  
    points    : Vec<Voxel>,
    sizes     : HashMap<usize,usize>,
    distances : Vec<((usize,usize),usize)>,
}

impl Space {
    fn new()->Self
    {
        Self 
        {
              points :     Vec::new(),
               sizes : HashMap::new(),
           distances :     Vec::new(),
        }
    }

    fn fill(&mut self,data:&[String])
    {
        let mut id = 0;

        self.points =
        data.iter().map(|line|
            {
                self.sizes.insert(id, 1);
                id+=1;
                Voxel::from_str(line,id-1)
            }
        ).collect();

        for i in 0..self.points.len()
        {
            for j in i+1..self.points.len()
            {
                self.distances.push( ((i,j),self.points[i].pos.dist_no_square(&self.points[j].pos)) );                
            }
        }

        self.distances.sort_by(|a,b| a.1.cmp(&b.1) );
    }
     
    fn count(&mut self,con:usize)->usize
    {     
        let mut conections = 0;      

        for x in 0..self.distances.len()
        {
            let ((i,j),_) = self.distances[x];

            if self.points[i].id != self.points[j].id            
            {
                let new_id = std::cmp::min(self.points[i].id,self.points[j].id);
                let old_id = std::cmp::max(self.points[i].id,self.points[j].id);

                let id1_count = *self.sizes.get(&self.points[i].id).unwrap();
                let id2_count = *self.sizes.get(&self.points[j].id).unwrap();

                for p in self.points.iter_mut()
                {
                    if p.id == old_id
                    {
                        p.id = new_id;
                    }
                }

                self.sizes.insert(new_id, id1_count + id2_count);
                self.sizes.insert(old_id, 0);
                
                if id1_count + id2_count == self.points.len() 
                {
                    return (self.points[i].pos.x*self.points[j].pos.x) as usize;
                }
                
                conections += 1;
                if x==con || conections == con-1
                {
                    break;
                }
            }
        }
        
        let mut sizes = self.sizes.values().cloned().collect::<Vec<usize>>();
        sizes.sort();
        sizes.iter().rev().take(3).product::<usize>()        
    }
}

fn part1(data:&[String],con:usize)->usize
{
    let mut w = Space::new();
    w.fill(data);
    w.count(con)
}

fn part2(data:&[String])->usize
{
    let mut w = Space::new();
    w.fill(data);
    w.count(usize::MAX)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day8");
    println!("part1: {}",part1(data,1000));
    println!("part2: {}",part2(data));    
}

#[allow(unused)]
fn get_test_data()->Vec<String>
{
    vec![
        "162,817,812".to_string(),
        "57,618,57".to_string(),
        "906,360,560".to_string(),
        "592,479,940".to_string(),
        "352,342,300".to_string(),
        "466,668,158".to_string(),
        "542,29,236".to_string(),
        "431,825,988".to_string(),
        "739,650,466".to_string(),
        "52,470,668".to_string(),
        "216,146,977".to_string(),
        "819,987,18".to_string(),
        "117,168,530".to_string(),
        "805,96,715".to_string(),
        "346,949,466".to_string(),
        "970,615,88".to_string(),
        "941,993,340".to_string(),
        "862,61,35".to_string(),
        "984,92,344".to_string(),
        "425,690,689".to_string()
    ] 
}

#[test]
fn test1()
{
    let v = get_test_data();
    assert_eq!(part1(&v,10),40);
}

#[test]
fn test2()
{
    let v = get_test_data();
    assert_eq!(part2(&v),25272);
}
