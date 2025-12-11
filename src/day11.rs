use std::collections::HashMap;
use lru_cache::LruCache;

#[derive(Clone,Debug)]
struct Node
{
    neigh : Vec<usize>,
    count : usize,    
}

impl Node {
    fn new(neigh:&[usize])->Self
    {
        Node 
        {
            neigh : neigh.to_vec(),
            count : 0,
        }
    }
}

struct Solve
{
     graph : HashMap<usize,Node>,
      memo : LruCache<(usize, bool, bool), usize>,
    you_id : usize,
    out_id : usize,
    fft_id : usize,
    dac_id : usize,
    svr_id : usize,
}

impl Solve {
    fn new(data:&[String])->Self
    {
        let mut graph = HashMap::new();
        let mut mapping : HashMap<String,usize> = HashMap::new();
        mapping.insert("you".to_string(), 0);
        mapping.insert("out".to_string(), 1);

        let find_id = |name:&str, mapping:&mut HashMap<String,usize>| -> usize {
            if !mapping.contains_key(name) {
                let id = mapping.len();
                mapping.insert(name.to_string(), id);
                id
            } else {
                *mapping.get(name).unwrap()
            }
        };

        for line in data.iter()
        {
            let l = line.split(": ").collect::<Vec<&str>>();
            let ids = l[1].split(' ')
                                      .map(|n| find_id(n, &mut mapping)).collect::<Vec<usize>>();

            graph.insert(find_id(l[0], &mut mapping), Node::new(&ids));
        };

        graph.insert(find_id("out", &mut mapping), Node::new(&Vec::new()));  

        Solve 
        {
            graph,            
            memo: LruCache::new(100000),
            you_id: find_id("you", &mut mapping),
            out_id: find_id("out", &mut mapping),
            fft_id: find_id("fft", &mut mapping),
            dac_id: find_id("dac", &mut mapping),
            svr_id: find_id("svr", &mut mapping),
        }
    }

    fn count(&mut self,from:usize,to:usize)->usize
    {
        let mut stack = vec![from];
       
        while let Some(node_name) = stack.pop()
        {
            if !self.graph.contains_key(&node_name)
            {
                continue;
            }

            let node = self.graph.get(&node_name).unwrap().clone();
            
            for neigh_name in node.neigh.into_iter()
            {
                if self.graph.get_mut(&neigh_name).is_some()
                {
                    let neigh_node = self.graph.get_mut(&neigh_name).unwrap();
                    neigh_node.count += 1;

                    stack.push(neigh_name);
                    if neigh_node.count>999999 { return 1 };
                }
            }
        }
        
        self.graph.get(&to).unwrap().count
 
    }    

    fn count22(&mut self,from:usize,fft:bool,dac:bool)->usize
    {
        if self.memo.contains_key(&(from,fft,dac))
        {
            return *self.memo.get_mut(&(from,fft,dac)).unwrap();
        }

        if from==self.out_id
        {           
            return if fft && dac { 1 } else { 0 };
        }

        let mut res = 0;

        let neig = self.graph.get(&from).unwrap().neigh.clone();  

        for &n in neig.iter()
        {
             res += self.count22(n,
                                 fft || n==self.fft_id,
                                 dac || n==self.dac_id);
        }     

        self.memo.insert((from,fft,dac), res);
        res
    }    

    fn count1(&mut self)->usize
    {
        self.count(self.you_id,self.out_id)
    }
    
    fn count2(&mut self)->usize
    {
        self.count22(self.svr_id,false,false)
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

