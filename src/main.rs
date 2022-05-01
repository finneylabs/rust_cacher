use std::{collections::HashMap};
use std::thread;
use std::time::Duration;


struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    data: HashMap<u32,u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            data: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.data.get(&arg) {
            Some(result) => result.clone(),
            None => {
                let newVal = (self.calculation)(arg);
                self.data.insert(arg, newVal);
                newVal
            }
        }
    }
}

fn main() {
    let expensive_closure = |num| {
        println!("Calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut cacher = Cacher::new(expensive_closure);
    println!("result = {}", cacher.value(34));
    println!("result = {}", cacher.value(34));
    println!("result = {}", cacher.value(35));
    println!("result = {}", cacher.value(34));
}
