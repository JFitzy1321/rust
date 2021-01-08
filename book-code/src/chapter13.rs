#[allow(dead_code)]
pub mod section1 {
    use core::hash::Hash;
    use std::time::Duration;
    use std::{collections::HashMap, thread};

    // fn simulated_expensive_calculation(intensity: u32) -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     intensity
    // }

    struct Cacher<T, K, V>
    where
        T: Fn(&K) -> V,
        K: Hash + Eq + Clone,
        V: Clone,
    {
        closure: T,
        values: HashMap<K, V>,
    }

    impl<T, K, V> Cacher<T, K, V>
    where
        T: Fn(&K) -> V,
        K: Hash + Eq + Clone,
        V: Clone,
    {
        fn new(closure: T) -> Self {
            Self {
                closure,
                values: HashMap::new(),
            }
        }

        fn value(&mut self, arg: &K) -> V {
            match self.values.get(arg) {
                Some(val) => val.clone(),
                None => {
                    let result = (self.closure)(arg);
                    let _ = self.values.insert(arg.clone(), result.clone());
                    result
                }
            }
        }
    }

    fn generate_workout(intensity: &u32, random_number: &u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly... ");
            thread::sleep(Duration::from_secs(2));
            *num
        });
        // let expensive_closure = |num| {
        //     println!("calculating slowly...");
        //     thread::sleep(Duration::from_secs(2));
        //     num
        // };

        if *intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(intensity));
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            match random_number {
                3 => println!("Take a break today! Remember to stay hydrated!"),
                _ => println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                ),
            }
        }
    }

    pub fn main() {
        let simulated_user_value = 10;
        let simulated_random_number = 7;

        generate_workout(&simulated_user_value, &simulated_random_number);
    }
}

#[allow(dead_code)]
pub mod section2 {
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Self {
            Self { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 20 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    pub fn main() {
        // let v1 = vec![1, 2, 3];

        // let v1_iter = v1.iter();
        // println!("{:?}", v1);

        // for val in v1_iter {
        //     println!("Got {}", val);
        // }

        let counter = Counter::new();

        for val in counter {
            println!("{}", val);
        }
    }
}
