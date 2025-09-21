use std::io;
use rand;
//use rand::prelude::*;
use std::env;
use std::fs;

#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle{
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}

struct Color(u8, u8, u8);   //RGB values
struct Point(u8, u8, u8);    //XYZ values

fn get_y(p: Point) -> u8 {
    p.1
}

fn main() {
    //Lab 2
    /*// Task 4
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
    temp * 1.8 + 32.0 // Implicit return in Rust (no need for `return` keyword)*/

    //Lab 4
    //Ch 8. Modules
    let mut buffer = String::new();
    println!("Enter a message: ");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);

    let number: i32 = buffer.trim().parse().unwrap(); //Also parse::<i32> would work
    println!("number + 1 is {}", number + 1);

    /*In video, rand = "0.8.0" was added to toml
    rand = "=0.9.2" was added to this toml
    */
   let number = rand::random::<f64>();
   println!("number is {}", number);
    
    //let number = thread_rng().gen_range(1..11); //May be listed in the new version
    //println!("number is {}", number);

    //Ch 9. Input and Output
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguements"); //Prevents errors for line 108
        return;
    }
    for (index, arguement) in env::args().enumerate() {
        println!("arguement {} is {}", index, arguement);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);


    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents is {}", contents);

    for line in contents.lines() {
        println!("line is {}", line);
    }

    //let contents = fs::read("planets.txt").unwrap();    //Reads as vector
    //println!("contents is {:?}", contents);   //THIS IS SO MUCH IT WORKS I SWEAR

    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other thins,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    //fs::write("speech.txt", speech);    //Will replace contents of existing files


    //Ch. 10 Structs
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 0.0
    };
    
    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle
    };
    
    let vehicle3 = Shuttle::new("Jerry");

    //println!("name is {}", vehicle.name);

    let vehicle_name = vehicle.get_name();  //Utilizes impl Shuttle
    println!("vehicle_name is {}", vehicle_name);

    vehicle.name = String::from("Atlantis");    //Can change specific aspects like this in Structs
    vehicle.crew_size = 6;
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(10000.0);
    println!("propellant is {}", vehicle.propellant);

    println!("name is {}", vehicle3.name);


    let red = Color(255, 0, 0);
    println!("First value is {}", red.0);

    let coord = Point(4, 5, 6); //Must be included, next line will be read for red instead
    let y = get_y(coord);
    println!("y is {}", y);
}
