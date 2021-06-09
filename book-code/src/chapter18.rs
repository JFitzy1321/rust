pub fn main() {
    if_let::main();
    pattern_syntax::main();
}

mod if_let {
    pub fn main() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }

        let mut stack = vec![];

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            // Loop will end when None is passed
            println!("{}", top);
        }

        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }

        println!("{:?}", v);
    }
}

mod pattern_syntax {
    pub fn main() {
        // matching literals
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }

        // matching named variabkes
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println! {"Got 50"},
            // this y is scoped to the match expression, doesn't shadow 'let y' above
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);

        // multiple patterns
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }

        // ranges pattern
        let x = 5;
        match x {
            // 1..5 is an exclusive range, 1..=5 is an inclusive range
            // only allowed for numerics and chars
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }

        // char range match
        let x = 'c';
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }
}
