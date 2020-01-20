// Remember: everything in Rust is private be default
// Use 'pub' keywora to make things public
pub struct AveragedCollection {
    list: Vec<i32>, // private member
    average: f64,   // private memebr
}

// To create a 'Class' we need to 'impl'ement functionality on data
// First, define data types: a.k.a. the struct
// Second, implement some methods on that struct
impl AveragedCollection {
    // 'Static' method, no reference to 'self' (thank you python knowledge)
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        // not including ';' at the last line of method
        // implies a return value
        self.average
    }

    // private method
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
