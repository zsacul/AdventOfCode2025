//use std::{thread::sleep, time::Duration};
use timer::Timer;

mod tools;
//mod pixoo;
//mod pixoocam;
mod dijkstria;
//use std::thread;
//mod cycliclist;
//mod cyclic2;
mod timer;
mod vec2;
mod vec3;
mod vec3f;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
//mod day07vis;
mod day08;
mod day09;
mod day10;
mod day11;
//mod day12;

//use divoom::*;
//use tokio; // Add this if not already in your dependencies



#[tokio::main]
async 
fn main() 
{

/*
    let num = pixoo::pixoo_test_text().await; 
    println!("pixoo_test_text: {:?}", num);
    return
*/

/*
    {
        let _timer = Timer::new();
        let day1_data = tools::read_1d_string("data/day01.txt");
        day01::solve(&day1_data);
    }


    {
        let _timer = Timer::new();
        let day2_data  = tools::read_1d_string("data/day02.txt");
        day02::solve(day2_data[0].as_str());
    }

    {
        let _timer = Timer::new();
        let day3_data  = tools::read_1d_string("data/day03.txt");
        day03::solve(&day3_data);
    }

    {
        let _timer = Timer::new();
        let day4_data  = tools::read_1d_string("data/day04.txt");
        day04::solve(&day4_data);
    }

    {
        let _timer = Timer::new();
        let day5_data  = tools::read_1d_string("data/day05.txt");
        day05::solve(&day5_data);
    }


    {
        let _timer = Timer::new();
        let day6_data  = tools::read_1d_string("data/day06.txt");
        day06::solve(&day6_data);
    }

//  {
//      let _timer = Timer::new();
//      let day7_data  = tools::read_1d_string("data/day07.txt");
//      day07::solve(&day7_data);
//      day07vis::solve(&day7_data).await
//  }

    {
        let _timer = Timer::new();
        let day8_data  = tools::read_1d_string("data/day08.txt");
        day08::solve(&day8_data);
    }

    {
        let _timer = Timer::new();
        let day9_data  = tools::read_1d_string("data/day09.txt");
        day09::solve(&day9_data);
    }  

    {
        let _timer = Timer::new();
        let day10_data  = tools::read_1d_string("data/day10.txt");
        day10::solve(&day10_data);
    }
*/     
    {
        let _timer = Timer::new();
        let day11_data  = tools::read_1d_string("data/day11.txt");
        day11::solve(&day11_data);
    }
/*                            
    {
        let _timer = Timer::new();
        let day12_data  = tools::read_1d_string("data/day12.txt");
        day12::solve(&day12_data);
    }

    {
        //let _timer = Timer::new();
        //let day21_data  = tools::read_1d_string("data/day21.txt");
        //day21::solve(&day21_data);

        let child = thread::Builder::new().stack_size(132 * 1024 * 1024).spawn(move || { 
            let _timer = Timer::new();
            let day21_data  = tools::read_1d_string("data/day21.txt");
            day21::solve(&day21_data);       
        }).unwrap(); 
        child.join().unwrap();
    }
*/
}
