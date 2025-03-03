use rand;
use std::io;
use std::env;
use std::fs;
use std::mem;
use std::ops::Add;
use std::fmt;

pub fn chal_one(){
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;
    
    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average,45.1); // other flaots needs to account for floating point precision. something like 
    println!("test 1 passed");
}

fn celsius_to_fahrenheit(x: f64) -> f64{ // chall two function
    x * 9.0 / 5.0 + 32.0
}

pub fn chal_two(){
    let conv = celsius_to_fahrenheit(13.0);

    assert_eq!(conv, 55.4);
    println!("test 2 passed");

}

pub fn chal_three(){
    let numbers = [1, 9 , -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];

    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0]; 
    let mut mean: f64 = 0.0;

    for item in numbers{
        if min > item{
            min = item;
        }
        if max < item{
            max = item;
        }
        mean += item as f64;
    }
    mean = mean / (numbers.len() as f64);

    assert_eq!(max, 56);
    assert_eq!(min,-18);
    assert_eq!(mean, 12.5);
    println!("test 3 passed");
}

pub fn chal_four_trim_spaces( var : & str) -> &str{
    let mut start: usize = 0;
    let mut end = 0;
    let word = false;
    for (item, value) in var.chars().enumerate(){
        if (value == ' ' ){
            start += 1;
        }
        else{
            break;
        }
    }
    
    for (item, value) in var.chars().rev().enumerate(){
        if (value == ' ' ){
            end += 1;
        }
        else{
            break;
        }
    }
    return &var[start..var.len()-end];

}

pub fn chal_five_higher_lower(){
    let random : u32 = rand::random_range(1..101);
    println!("Enter a number between 1 and 100");

    loop {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input).expect("Failed to read input line");
        let inputNum : u32 = input.trim().parse().expect("Failed to parse guess");

        if (inputNum < random){
            println!("\nHigher");
        }
        else if (inputNum > random){
            println!("\nLower");
        }
        else {
            break;
        }
    }


    println!("Congrats you correctly guessed the number was {random}")

}

pub fn chal_six_check_rost(mut args : env::Args){
    // first arg is file, second arg is name to be found in the file.
    if (args.len() < 2){
        println!("2 arguments are required");
        return;
    }
    let file_name = args.nth(1).unwrap();
    println!("{file_name}");
    let name = args.nth(0).unwrap();

    let file_string = fs::read_to_string(file_name).unwrap();

    if (file_string.contains(&name)){
        println!("{name} is in roster!");
    }
    else{
        println!("{name} is not in roster!");
    }

}

struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn get_area(&self) -> f64{
        self.height * self.width
    }
    fn scale(&mut self, scale : f64){
        self.height = self.height * scale;
        self.width = self.width * scale;
    }
    fn new(height: f64, width: f64) -> Rectangle{
        Rectangle {
            height,
            width
        }
    }
}

pub fn chal_seven_rectangle(){
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(),1.02);
    println!("Tests passed!");
}

pub fn chal_eight_boxes<T: Add<Output = T>>(num1 : Box<T>, num2 : Box<T>) -> Box<T>{
    Box::new(*num1 + *num2)
}

impl fmt::Display for Rectangle{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>{
        let height = self.height;
        let width = self.width;
        write!(f, "height: {height} \t width: {width}")
    }
}

pub fn chal_nine_traits(){
    let rect : Rectangle = Rectangle{
        width: 2.3,
        height: 1.3
    };
    println!("{rect}");

}

enum Location {
    Unknown,
    Anonymous,
    Known(f64,f64)
}

impl Location {
    fn display(&self) -> String{
        match &self {
            Location::Unknown => String::from("Location is unknown"),
            Location::Anonymous => String::from("Tor browser"),
            Location::Known(lat,long) => format!("{lat} - {long}")
        }
    }
}

pub fn chal_ten_enums(){
    let a = Location::Unknown;
    println!("{}", a.display());
    let b = Location::Anonymous;
    println!("{}", b.display());
    let c = Location::Known(10.0, 20.3);
    println!("{}", c.display());

}