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

    // 10-8 A Point<T, U> generic over two types so that x and y can be values of different types ----
    {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };

        // You can use as many generic type parameters in a definition as you want,
        // but using more than a few makes your code hard to read.
        // If you’re finding you need lots of generic types in your code,
        // it could indicate that your code needs restructuring into smaller pieces.
    }

    // In Enum definitions
    {
        // The Option<T> enum:
        enum Option<T> {
            Some(T),
            None,
        }

        // The Result<T, E> enum:
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }

    // 10-9 Implementing a method named x on the Point<T> struct that will return a reference to the x field of type T ----
    {
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());
    }

    // 10-10 An impl block that only applies to a struct with a particular concrete type for the generic type parameter T ----
    {
         struct Point<T> {
            x: T,
            y: T,
        }

        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2).sqrt())
            }
        }
    }

    // 10-11 A method that uses generic types that are different from its struct's definition ----
    {
        struct Point<X1, Y1> {
            x: X1,
            y: Y1,
        }

        impl<X1, Y1> Point<X1, Y1> {
            fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

}
