pub mod panic {
    #[allow(dead_code)]
    pub fn main() {
        // panic!("Crash and Burn");
        let v = vec![1, 2, 3];

        v[99];
    }
}

pub mod result {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    // pub fn main() {
    //     // let file = File::open("hello.txt");
    //     // let _ = match f {
    //     // Ok(file) => file,
    //     //     Err(e) => match e.kind() {
    //     //         ErrorKind::NotFound => match File::create("hello.txt") {
    //     //             Ok(fc) => fc,
    //     //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //     //         },
    //     //         other_error => {
    //     //             panic!("Problem opening the file: {:?}", other_error)
    //     //         }
    //     //     },
    //     // };
    //     let _ = File::open("hello.txt").unwrap_or_else(|error| {
    //         if error.kind() == ErrorKind::NotFound {
    //             File::create("hello.txt").unwrap_or_else(|error| {
    //                 panic!("Problem creating the file: {:?}", error);
    //             })
    //         } else {
    //             panic!("Problem opening the file: {:?}", error);
    //         }
    //     });
    // }

    #[allow(dead_code)]
    pub fn main() -> Result<String, io::Error> {
        // let f = File::open("hello.txt");

        // let mut f = match f {
        //     Ok(file) => file,
        //     Err(e) => return Err(e),
        // };

        // let mut s = String::new();

        // match f.read_to_string(&mut s) {
        //     Ok(_) => Ok(s),
        //     Err(e) => Err(e),
        // }

        // let mut file = File::open("hello.txt")?;
        // let mut s = String::new();
        // file.read_to_string(&mut s)?;
        // Ok(s)
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
}
