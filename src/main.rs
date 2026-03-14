fn main() {

    // 10-1 Finding the largest number in a list of numbers ----
    {
        let number_list = vec![32, 50, 25, 100, 65];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {largest}");
    }

    // 10-2 Code to find the largest number in two lists of numbers ----
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {largest}");

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {largest}");
    }

    // 10-3 Abstracted code to find the largest number in two lists ----
    {
        fn largest(list:&[i32]) -> &i32 {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {result}");

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let result = largest(&number_list);
        println!("The largest number is {result}");
    }

    // 10-4 Two functions that differ only in their names and in the types in their signatures ----
    {
        fn largest_i32(list: &[i32]) -> &i32 {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        fn largest_char(list: &[char]) -> &char {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {result}");
    }

    // 10-5 The largest function using generic type parameters; this doesn't compile yet ----
    {
        // fn largest<T>(list: &[T]) -> &T {
        //     let mut largest = &list[0];

        //     for item in list {
        //         if item > largest { // Error: consider restricting type parameter `T` with trait `PartialOrd`
        //             largest = item;
        //         }
        //     }

        //     largest
        // }

        // let number_list = vec![34, 50, 25, 100, 65];

        // let result = largest(&number_list);
        // println!("The largest number is {result}");

        // let char_list = vec!['y', 'm', 'a', 'q'];

        // let result = largest(&char_list);
        // println!("The largest char is {result}");
    }

    // 10-6 A Point<T> struct that holds x and y values of type T ----
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point { x: 5, y: 10 };  // Point<i32>
        let float = Point { x: 1.0, y: 4.0 }; // Point<f64>
    }

    // 10-7 The fields x and y must be the same type because both have the same generic data type T ----
    {
        // struct Point<T> {
        //     x: T,
        //     y: T,
        // }

        // let wont_work = Point { x: 5, y: 4.0 };
    }

    

}
