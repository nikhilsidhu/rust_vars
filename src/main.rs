fn main() {

    // ---- VARIABLES ----- //
    // variables immutable by default
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // can shadow variables
    let y = 2;
    println!("The value of y is: {}", y);
    let y = y + 1;
    println!("The value of y is: {}", y);

    {
        // error thrown without 'let; b/c y is not mutable
        let y = y * 3; // shadowing y effectively creates new var
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    // f32 -> 32bit float (single precision)
    // f64 -> 64bit float (double precision) DEFAULT
    let float64 = 0.01;
    let float32: f32 = 0.25;
    println!("The float vlaues are: {}, {}.", float32, float64);

    // tuple -> groups multiple types into compound type;
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    let _a = tup.0; // index access
                    // _ -> for intentionally unused variable
    println!("The values are: {}, {}, {}.", x, y, z);

    // array -> fixed length, multiple values of one type
    let arr: [i32; 3] = [1, 2, 3];
    let _arr_threes = [3; 5]; // arr_threes === [3, 3, 3, 3, 3]
    let first_num = arr[0];
    println!("The first number is: {}", first_num);

    // cannot use mut with constants (always immutable)
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;    

    // rust is statically typed
    // - types can usually be inferred
    // - in cases where many types are possible type annotation req'd
    let _guess: u32 = "42".parse().expect("Not a number!");

    // rust doesn't check for integer overflow when compiling w/ --release
    // - values > max value wrap around to min
    // - this will throw an error compiling without --release
    // let too_big : u8 = 256; // u8 can hold 0 - 255
    // println!("The number is too big! -> {}", too_big);

    // char is the most primitive alphabetic type
    // char literals use ' '
    // literal === never mutable (the var is mutable but literal itself isn't)
    let _c = 'z';
    let _dog = '🐶';

    // ----- CONTROL FLOW ----- // 
    let condition = true;
    let _num = if condition { 5 } else { 6 }; // must be of the same type
    
    loop {
        println!("again!");
        break; // comment to go on forever...
    }

    let mut count = 0;
    // loops can have labels
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; // breaks the innermost loop its contained in
            }
            if count == 2 {
                break 'counting_up; // can specify loop to break!
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("final count = {}", count);

    let mut counter = 0;
    // can use variables to hold a loop (and use returned value)
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {}", result);

    let a = [10, 20, 30, 40, 50];
    // for-in loop
    for element in a {
        println!("value is {}", element);
    }
    // with Range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
