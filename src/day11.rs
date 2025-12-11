use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Node
{
    neigh:Vec<String>,
    count:usize,
    countf:usize,
    ok:usize,
    fft:bool,
    dac:bool,
}

impl Node {
    fn new(s:String)->Self
    {
        let n = s.split(' ').map(|x| x.to_string()).collect::<Vec<_>>();

        Node {
            neigh:n,
            count:0,
            countf:0,

            ok:0,
            fft:false,
            dac:false,
        }
    }

    fn zero(&mut self)
    {
        self.count = 0;
        self.countf = 0;

        self.ok = 0;
        self.fft = false;
        self.dac = false;
    }
}

struct Solve
{
    graph   : HashMap<String,Node>,
    result:usize,
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
            result:0,
        }
    }

    fn reset(&mut self)
    {
        for (_k,v) in self.graph.iter_mut()
        {
            v.zero();
        }
    }

    //2963
    fn count(&mut self,from:String,to:String)->usize
    {
        self.reset();
        let mut stack = vec![from];
       
        while let Some(node_name) = stack.pop()
        {
            if self.graph.get(&node_name).is_none()
            {
                continue;
            }
            let neig = self.graph.get(&node_name).unwrap().neigh.clone();  
            
            for neigh_name in neig            
            {
                if self.graph.get_mut(&neigh_name).is_some()
                {
                    let neigh_node = self.graph.get_mut(&neigh_name).unwrap();
                    neigh_node.count += 1;

                    stack.push(neigh_name.to_string());
                    if neigh_node.count>999999 { return 1 };
                }
            }
        }
        
        self.graph.get(&to).unwrap().count
 
    }    


        //2963
        //2691
        //1

        //35556
        //408894
        //3733380
        //37138242
    fn count22(&mut self,from:String,to:String)->usize
    {
        self.reset();
        let mut stack = vec![(from.clone(),false,false)];
       
        while let Some((node_name, fft, dac)) = stack.pop()
        {
            if self.graph.get(&node_name).is_none()
            {
                continue;
            }

            if node_name=="out"
            {
                //println!("Visiting out {} fft:{} dac:{}",from,fft,dac); 
                if fft && dac {
                    self.result+=1;
                }
            }
            let neig = self.graph.get(&node_name).unwrap().neigh.clone();  
            
            for neigh_name in neig            
            {
                if self.graph.get_mut(&neigh_name).is_some()
                {
                    let neigh_node = self.graph.get_mut(&neigh_name).unwrap();
                    neigh_node.count += 1;

                    stack.push((neigh_name.to_string(), fft || node_name.to_string()=="fft", 
                                                        dac || node_name.to_string()=="dac"));
                    if neigh_node.count>999999999 { return 1 };
                }
            }
        }
        
        self.graph.get(&to).unwrap().count
 
    }    

    fn count1(&mut self)->usize
    {
        self.count("you".to_string(),"out".to_string())
    }

    //2963
    fn countFFTDAC(&mut self,from:String,to:String)->usize
    {
        self.reset();
        let mut stack = vec![from];
       
        while let Some(node_name) = stack.pop()
        {
            if self.graph.get(&node_name).is_none()
            {
                continue;
            }
            let neig = self.graph.get(&node_name).unwrap().neigh.clone();  
            let fft = self.graph.get(&node_name).unwrap().fft;
            let dac = self.graph.get(&node_name).unwrap().dac;
            
            for neigh_name in neig            
            {
                if self.graph.get_mut(&neigh_name).is_some()
                {
                    let neigh_node = self.graph.get_mut(&neigh_name).unwrap();
                    
                    neigh_node.fft = neigh_node.fft || node_name=="fft" || fft;
                    neigh_node.dac = neigh_node.dac || node_name=="dac" || dac;

                    if neigh_node.dac && neigh_node.fft
                    {
                        neigh_node.count += 1;
                    }

                    if neigh_node.count>990000 { return 1};
                    stack.push(neigh_name.to_string());
                }
            }
        }
        
        self.graph.get(&to).unwrap().count
 
    }      

    fn dfs(&mut self,node:String,fft:bool,dac:bool,visited:&mut HashSet<String>)->usize
    {
        if self.graph.get(&node).is_none() { return 0; }

        //if visited.len()<60 { 
        //}
        
        if node=="out"
        {
            if  fft && dac { 
                println!("Visiting: {} fft:{} dac:{} visited:{:?}",node,fft,dac,visited);
                return 1; 
            } else { return 0; }
        }

        if visited.contains(&node)
        {
            return 0;
        }
        visited.insert(node.clone());

        let neig = self.graph.get(&node).unwrap().neigh.clone();
   //     let fft = self.graph.get(&node).unwrap().fft;
     //   let dac = self.graph.get(&node).unwrap().dac;

        let mut count = 0;
        for n in neig.iter()
        {
            
            count += self.dfs(n.to_string(),fft || n.to_string()=="fft",dac || n.to_string()=="dac", visited);
        }

        
        {
            let nn = self.graph.get_mut(&node).unwrap();
            nn.count -= 1;
        }
        visited.remove(&node);
        count

    }
 
    
    
    fn count2(&mut self)->usize
    {
        self.result = 0;
        self.count22("svr".to_string(),"out".to_string());
        self.result
//        let mut visited = HashSet::new();
//        self.dfs("svr".to_string(),false,false,&mut visited)
       // self.countFFTDAC("svr".to_string(),"out".to_string())
//        self.count("svr".to_string(),"fft".to_string())*
//        self.count("fft".to_string(),"dac".to_string())*
//        self.count("dac".to_string(),"out".to_string())        
//        +
//        self.count("svr".to_string(),"dac".to_string())*
//        self.count("dac".to_string(),"fft".to_string())*
//        self.count("fft".to_string(),"out".to_string())
//        -
//        self.count("svr".to_string(),"fft".to_string())*
//        self.count("fft".to_string(),"dac".to_string())
//        -
//        self.count("svr".to_string(),"fft".to_string())*
//        self.count("fft".to_string(),"dac".to_string())
    }

}

fn part1(data:&[String])->i64
{
    Solve::new(data).count1() as i64
}

fn part2(data:&[String])->i64
{
    Solve::new(data).count2() as i64
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
        "svr: aaa bbb".to_string(),
        "aaa: fft".to_string(),
        "fft: ccc".to_string(),
        "bbb: tty".to_string(),
        "tty: ccc".to_string(),
        "ccc: ddd eee".to_string(),
        "ddd: hub".to_string(),
        "hub: fff".to_string(),
        "eee: dac".to_string(),
        "dac: fff".to_string(),
        "fff: ggg hhh".to_string(),
        "ggg: out".to_string(),
        "hhh: out".to_string(),
    ];
    assert_eq!(part2(&v),2);
}

