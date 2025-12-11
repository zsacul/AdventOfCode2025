use std::collections::HashMap;

#[derive(Debug)]
struct Node
{
    neigh:Vec<String>,
    count:usize
}

impl Node {
    fn new(s:String)->Self
    {
        let n = s.split(' ').map(|x| x.to_string()).collect::<Vec<_>>();

        Node {
            neigh:n,
            count:0,
        }
    }
}

struct Solve
{
    graph   : HashMap<String,Node>,
}

impl Solve {
    fn new(data:&[String])->Self
    {
        let mut graph = HashMap::new();

        for line in data.iter()
        {
            let l = line.split(": ").collect::<Vec<&str>>();
            graph.insert(l[0].to_string(), Node::new(l[1].to_string()));                    
        };

        graph.insert("out".to_string(), Node::new("".to_string()));                    

        Solve {
            graph,            
        }
    }

    fn count1(&mut self)->usize
    {
        let mut stack = vec!["you".to_string()];
     //   println!("{:#?}", self.graph);
        
        while let Some(node_name) = stack.pop()
        {
            if self.graph.get(&node_name).is_none()
            {
                continue;
            }
            let neig = self.graph.get(&node_name).unwrap().neigh.clone();

            println!("Visiting node: {}, neighbors: {:?}", node_name, neig);
            
            for neigh_name in neig            
            {
                if self.graph.get_mut(&neigh_name).is_some()
                {
                    //add 1 to self.graph.get(neigh_name).count
                    let neigh_node = self.graph.get_mut(&neigh_name).unwrap();
                    neigh_node.count += 1;
                    stack.push(neigh_name.to_string());
                }
            }
        }
        println!("{:#?}", self.graph);
        self.graph.get("out").unwrap().count
 
    }

}

fn part1(data:&[String])->i64
{
    Solve::new(data).count1() as i64
}

fn part2(data:&[String])->i64
{
    0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day11");
    println!("part1: {}",part1(data));
    println!("part2: {}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "aaa: you hhh".to_string(),
        "you: bbb ccc".to_string(),
        "bbb: ddd eee".to_string(),
        "ccc: ddd eee fff".to_string(),
        "ddd: ggg".to_string(),
        "eee: out".to_string(),
        "fff: out".to_string(),
        "ggg: out".to_string(),
        "hhh: ccc fff iii".to_string(),
        "iii: out".to_string(),
    ];
    assert_eq!(part1(&v),5);
}

#[test]
fn test2()
{
    let v = vec![

    ];
    assert_eq!(part2(&v),3121910778619);
}

