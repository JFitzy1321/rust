pub fn main() {
    if_let::main();
    pattern_syntax::main();
    destructuring::main();
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

        // ignoring values with '_'
        fn foo(_: i32, y: i32) {
            // This seems really contrived, but whatever. It's trying to make an example
            println!("This code only uses the y parameter: {}", y)
        }

        foo(3, 4);

        // ignoring parts with a nested '_'
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => println!("Can't override an existing customized value"),
            _ => setting_value = new_setting_value,
        }

        println!("setting is {:?}", &setting_value);

        let numbers = (1, 2, 3, 4, 5);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            }
        }

        let s = Some("hello".to_string());

        // doesn't move s
        if let Some(_) = s {
            println!("Found a string")
        }

        // moves s in _s, but doesn't bind the value to _s
        if let Some(_s) = s {
            println!("Found another string")
        }

        // ignore values with .. in destructuring
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            // this is technically an exhaustive list (i.e. an 'irrefutable pattern')
            Point { x, .. } => println!("x is {}", x),
        }

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => println!("Some numbers: {}, {}", first, last),
        }

        // match guards
        let num = Some(4);
        match num {
            // only matches if the pattern and conditional are true
            Some(x) if x < 5 => println!("less than five: {}", x),
            // matches if the pattern is true but the conditional above is false
            Some(x) => println!("{}", x),
            None => (), // unit type (), ie do nothing
        }

        // compare variables in match to outer scope variables with match guard
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            // y is not a part of a pattern, therefore is not shadowed
            // this is using y in the outer scope, but n is in this arm's scope
            Some(n) if n == y => println!("Matched, n = {}", n),
            _ => println!("Default cause, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {}", x, y);

        enum Message {
            Hello { id: i32 },
        }

        // use @ to capture value in pattern
        let msg = Message::Hello { id: 5 };
        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}

mod destructuring {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
        ChangeColor2(Color),
    }

    pub fn main() {
        let p = Point { x: 0, y: 7 };
        // destructure x and y in a and b, respectively
        let Point { x: a, y: b } = p;

        assert_eq!(0, a);
        assert_eq!(7, b);

        // destructure x and y into variables
        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        // match with struct destructuring
        let p = Point { x: 1, y: 7 };
        match p {
            Point { x, y: 0 } => println!("On the x axis at {:?}", (x, y)),
            Point { x: 0, y } => println!("On the y acis at {:?}", (x, y)),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }

        // destructuring enums

        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            // Quit has nothing to destructure
            Message::Quit => println!("The Quit variant has no data to destructure"),
            // struct like enum
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y)
            }
            // Single field enum
            Message::Write(text) => println!("Text message: {}", text),
            // tuple like enum
            Message::ChangeColor(r, g, b) => {
                println!("Change hte color to red {}, green {}, and blue {}", r, g, b)
            }
            // Destructuring nested values "objects"
            Message::ChangeColor2(Color::Rgb(r, g, b)) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b)
            }
            Message::ChangeColor2(Color::Hsv(h, s, v)) => println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            ),
        }
    }
}
