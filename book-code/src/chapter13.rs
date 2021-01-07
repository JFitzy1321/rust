pub mod section1 {
    use std::collections::HashMap;
    use std::thread;
    use std::time::Duration;
    // fn simulated_expensive_calculation(intensity: u32) -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     intensity
    // }
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        closure: T,
        value: HashMap<u32, u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(closure: T) -> Self {
            Self {
                closure,
                value: HashMap::new(),
            }
        }

        fn value(&mut self, arg: &u32) -> u32 {
            match self.value.get(arg) {
                Some(v) => *v,
                None => {
                    let v = (self.closure)(*arg);
                    let _ = self.value.insert(*arg, v);
                    v
                }
            }
        }
    }

    fn generate_workout(intensity: &u32, random_number: &u32) {
        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly... ");
            thread::sleep(Duration::from_secs(2));
            num
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
