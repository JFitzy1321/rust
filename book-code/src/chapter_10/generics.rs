// Code from the chapter synopsis
// pub fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     // let mut largest = number_list[0];

//     // for number in number_list {
//     //     if number > largest {
//     //         largest = number;
//     //     }
//     // }

//     // println!("The largest number is {}", largest);

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
// }

// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
#[allow(dead_code)]
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
#[allow(dead_code)]
fn print_with_typename<T: std::fmt::Debug>(var: &T) {
    println!("The largest {} is {:?}", std::any::type_name::<T>(), var);
}

#[allow(dead_code)]
pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);
    let result = largest(&number_list);
    print_with_typename(result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    let result = largest(&char_list);
    print_with_typename(result);
}
