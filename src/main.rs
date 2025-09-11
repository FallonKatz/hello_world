fn main() {
    // Task 4
    println!("Hello, world!");

    // Ch 2. Primitive Data Types
    let mut x = 10; // Mutable variable
    println!("x is {}", x);
    x = 20; // Now it works!
    println!("x is {}", x);

    let xy = -10; // Can be signed (i32 by default)
    println!("xy is {}", xy);

    let y = 1.234567890123456; // f64 by default
    println!("y is {}", y);

    let z: f32 = 1.234567890123456;
    println!("z is {}", z);

    let a = 10;
    let b = 3.0;
    let c = a as f64 / b;
    println!("c is {}", c);

    let tr = true;
    let fa = false;
    println!("tr is {} and fa is {}", tr, fa);
    println!("not tr is {}", !tr);
    let trfa = (tr ^ fa) | (tr & fa);
    println!("trfa is {}", trfa);

    let a1 = 13;
    let b1 = 2.3;

    let average = (a1 as f64 + b1 + c) / 3.0;
    println!("average is {}", average);

    // Use approximate comparison for floats
    let expected = 6.2111;
    let tolerance = 0.0001;
    assert!((average - expected).abs() < tolerance, "Test failed: got {}, expected ~{}", average, expected);

    println!("Test passed!");

    // Ch 3. Compound Data Types
    let parking_lot = [[1, 2, 3], [4, 5, 6]];
    let number = parking_lot[1][2];
    print!("number is {}", number);

    //let garage =[[[0; 100]; 20]; 5];

    // Ch 4. Functions
    say_hello();

    let celsius_temp = 23.0;
    let far_temp = celsius_to_far(celsius_temp);

    // Use a tolerance for floating-point comparison
    let expected_far = 73.4;
    let tolerance = 0.1; // Allow a small difference due to floating-point precision
    assert!((far_temp - expected_far).abs() < tolerance, "Test failed: got {}, expected ~{}", far_temp, expected_far);

    println!("Test passed!");
}

fn say_hello() {
    print!("Hello!");
}

fn celsius_to_far(temp: f64) -> f64 {
    temp * 1.8 + 32.0 // Implicit return in Rust (no need for `return` keyword)
}
