use std::{collections::HashMap};
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T,V,K>
    where T: Fn(V) -> K
{
    calculation: T,
    data: HashMap<V,K>,
}

impl<T,V,K> Cacher<T,V,K>
    where T: Fn(V) -> K, V: Eq, V: Hash, V: Clone, K: Clone
{
    fn new(calculation: T) -> Cacher<T,V,K> {
        Cacher {
            calculation,
            data: HashMap::new()
        }
    }

    fn value(&mut self, arg: V) -> K {
        match self.data.get(&arg) {
            Some(result) => result.clone(),
            None => {
                let newVal = (self.calculation)(arg.clone());
                self.data.insert(arg, newVal.clone());
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

    let expensive_closure_string = |num| {
        println!("Calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut cacher = Cacher::new(expensive_closure_string);
    println!("result = {}", cacher.value(String::from("Guido")));
    println!("result = {}", cacher.value(String::from("Guido")));
    println!("result = {}", cacher.value(String::from("Guido")));
    println!("result = {}", cacher.value(String::from("Guido")));
    println!("result = {}", cacher.value(String::from("Jorge")));
    println!("result = {}", cacher.value(String::from("Jorge")));
    println!("result = {}", cacher.value(String::from("Guido")));
    println!("result = {}", cacher.value(String::from("Guido")));

    let expensive_closure_int_to_string = |num: u32| {
        println!("Calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        format!("{} signed", num.to_string())
    };
    let mut cacher = Cacher::new(expensive_closure_int_to_string);
    println!("result = {}", cacher.value(34));
    println!("result = {}", cacher.value(34));
    println!("result = {}", cacher.value(34));
    println!("result = {}", cacher.value(35));
    println!("result = {}", cacher.value(35));
    println!("result = {}", cacher.value(34));
}
