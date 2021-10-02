fn main() {
    // variable immutable by default
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // can shadow variables
    let y = 2;
    println!("The value of y is: {}", y);
    let y = y + 1;
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
}
