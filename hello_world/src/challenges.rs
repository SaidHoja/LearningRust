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