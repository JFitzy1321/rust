pub fn main() {
    println!("\nChapter16 - Threading");
    threading::main();

    println!("\nChapter16 - Channels");
    channels::main();

    println!("\nChapter16 - Mutex");
    mutex::main();
}

mod threading {
    use std::thread;
    // use std::time::Duration;

    pub fn main() {
        let v = vec![1, 2, 3];

        // This will 'move' v's ownership to the closure in the thread.
        // But what if I want a copy/clone, and use the original in my main thread, or other threads?
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        // let handle = thread::spawn(|| {
        //     for i in 1..=10 {
        //         println!("hi number {} from the spawned thread", i);
        //         thread::sleep(Duration::from_millis(1));
        //     }
        // });

        handle.join().unwrap();

        // for i in 1..=5 {
        //     println!("hi number {} from the main thread!", i);
        //     thread::sleep(Duration::from_millis(1));
        // }
    }
}

mod channels {
    use std::{sync::mpsc, thread, time::Duration};

    // basic setup for a transmitter - receiver channel
    // pub fn main() {
    //     let (tx, rx) = mpsc::channel();

    //     thread::spawn(move || {
    //         let val = String::from("hi");
    //         tx.send(val).unwrap();
    //     });

    //     let received = rx.recv().unwrap();
    //     println!("Receiver got: {}", received);
    // }

    // example of threas actually communicating
    pub fn main() {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                "hi".to_string(),
                "from".to_string(),
                "the".to_string(),
                "spawned".to_string(),
                "thread".to_string(),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                "more".to_string(),
                "messages".to_string(),
                "for".to_string(),
                "you".to_string(),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Main thread got: {}", received);
        }
    }
}

mod mutex {
    use std::sync::Mutex;
    // using mutex
    pub fn main() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }
}
