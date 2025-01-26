

fn main() {

}

/*
Chapter 1:
    Rust basics
    - Rust is a statically typed language, so all types must be known at compile time
    - Rust is a compiled language, so it must be compiled before it can be run




*/
/*
Rust basic types and variable declaration

Chapter 2

    i64, i32, i16, i8, u64, u32, u16, u8, usize, f64, f32
    let x : i32;
    char, string, bool, f64, f32

    No implicit conversions for safety I guess. 



*/

/*

    Chapter 3
    Arrays 
    let letters = ['a', 'b', 'c', 'd'];
    pretty standard
    same for retrieval letters[0];
        Length and data type are immutable, also standard
    also can do
    let numbers: [i32; 5]; , doing numbers[4] will error as it is unitilized, so no 0 initialization it seems
        Can manually do it by doing let numbers = [0; 5]; which will create an array of 5 elements all initialized to 0 ( the i32 got replaced with original value)
    Has compile time bounds checking, so no simple out of bounds errors
        but runtime errors are still possible. 
    Length operation return type is a usize type to allow for better target based compilation. 

    Can do multidimensional arrays
    Standard way of doing access and changes there too
    let matrix = [[1, 2], [3, 4]];
    let first_item = matrix[0][0];
    matrix[0][0] = 5; etc

    Inner dimensions must match. 

    Tuples

    can mix types
    fixed memory and size, contiguously stored in memory
    know data types at compile time
    let tup = (500, 6.4, 'a'); // remember to make mut in order to change values later if required
    let x = tup.0;
    tup.1 = 3.2;
    etc

    let (a,b,c) = tup;
    simple unrolling syntactic sugar

    standard zero indexing for all.

*/

/*
    Chapter 4
    functions
    keywords: fn 
    fn function_name(){
        // code here
    }
    if in different file/crate and want public use keyword pub
    need mod filename to import it if in different file
    import crate if in different crate i think
    pub fn function_name(){
        // code here
    }

    parameter types declared similar to variable declaration
    fn function_name(param1: i32, param2: i32){
        // code here
    }

    compiler will automatically infer types if not specified for function calls

    let x = 1;
    let y = 2;
    u8_function(x, y);
    i32_function(x, y); this will error as x and y are u8 types because of earlier inference from function call

    This is done because rust does not do implicit type conversion for safety reasons

    Can return at the end of functions just by having the last line be the return value / expr

    fn square (i32: x) -> i32{
        x * x
    }
    Must be last line

    Can also return early by using the return keyword
    fn square (i32: x) -> i32{
        if (x == 0){
            return 0;
        }
        return x * x
    }

    can also return tuples
    fn return_tuple() -> (i32, i32){
        (1, 2)
    }

    if a function returns nothing rust will automatically return a unit type represented by "()"

 */


 /*
    Chapter 5 flow control

    If statements look pretty standard
    if x + 1 != 3 {
        // code here
    } else if x == 4 {
        if y == 5 {
            // code here
        }
        // code here
    } else {
        // code here
    }
    don't need () it appears, looks a little awkward to me. 

    can use if statements as expr
    let x = if y == 5 { 10 } else if y == 4 { 32 } else { 15 }; 

    no ternary expression though :(

    Loops
    loop {
        // code here
        if (whatever){
            break;
        }
    }
    just an infinite loop thing i guess, can use break to exit

    Loops also have return values

    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    }; but need to end with semicolon now 

    while loops seem pretty standard

    while x < 5 {
        // code here
        x+=1; // no ++ or -- operators, so sad
    }

    Key difference between while and loop is while can't return a value but loop can 

    for loop, basically for each loops

    for x in 0..5 { // can do ranges
        // code here
    }
    for x in array { // or standard for each
        // code here
    }

    creates an iterator to do this with next(). 

    can do this to get position too

    for x in [1,2,3,4,5].iter().enumerate() {
        println!("{} {}", x.0, x.1);
    }
    or 
    for (index, item) in [1,2,34,5].iter().enumerate() {
        println!("{index}\t{item}");
    } because you can break tuples

    In this scenario item is the reference to the value in the array
    but without .iter() or .enumarate() its by value. 

    fixed with & operator to derefernce

        for (index, &item) in [1,2,34,5].iter().enumerate() {
        if (item == 2){
        do something
        }
    }

    now mutability and referencing 

    for row in matrix.iter_mut(){
        println!();
        for item in row.iter_mut(){
            *item += 10;
            print!("{}\t", item);
        }
    }




    can also do this stuff
        'outer: for x in 0..10 {
        if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
        'inner: for y in 0..10 {
            if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
            println!("x: {}, y: {}", x, y);
        }
    }

        'outer: for x in 1..10 {
        if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
        'inner: for y in 1..10 {
            if y % 2 == 0 { break 'inner; } // Continues the loop over `y`.
            println!("x: {}, y: {}", x, y);
        }
    }

    little odd 


  */

  /*
  Chapter 6 ownership

      var scope is pretty much the same, basically just inside the same braces.
      Scope is used to 

      Shadowing
      Declaring a variable with the same name as an existing one

        let x = 5;
        print!("{x}")
        let x = 7;
        print!("{x}")
        This is perfectly legal and will rust in 5 then 7 being printed

        but this is illegal

        let x = 5;
        print!("{x}")
        x = 7; // because x is not mut
        print!("{x}")
        Can also change the data type and everything else as it is a new variable, seems like bad practice to do this
    Doesn't modify it actually creates a new variable with the same name, the second will be used till it leaves scope.

    Memory 
    Stack and heap system similar to C

    Strings are heap allocated
    Strings are mutalble
    String::from("hello") creates a heap allocated string
    others like 
    let s = "ASDA" are literals so differently allocated

    let mut messages = String::from("Hello");
    messages.push_str(", world!"); //
    messages.push('!'); // can push a single character

    Ownership

    Rust values can only have one owner at a time
    When the owner goes out of scope the value is dropped, unless it is moved to another owner
    techniques to move ownership/values
        Moving
        for example

        let x : String;
        {
            let y = "sadf";
            x = y;
            print!("{y}") // this will error as y is no longer in scope as its ownership has been moved to x
        }
        // ints and things of that nature are copied (stack allocated variables), but heap allocated things are moved

        Cloning
               let x : String;
        {
            let y = "sadf";
            x = y.clone();
            print!("{y}") // this is now fine as the data is cloned and x and y own two seperate copies
        }

        Passing ownership to different functions

        fn main() {
            let x = String::from("hello");
            take_ownership(x);
            println!("{}", x); // this will error as x has been moved to the function
        }

        fn take_ownership(s: String){
            println!("{}", s);
        }

        can use clone again to easily due this. or you can return the value from the function

        fn main() {
            let x = String::from("hello");
            let x = take_ownership(x);
            println!("{}", x); 
        }

        fn take_ownership(s: String) -> String{
            println!("{}", s);
            s // this does not have to be the same string that was passed in.
        }

        Seems like the rules of ownership follow this for functions:

        Any heap allocated variable passed in is transfer ownership to the intake function
        any variable returned from a function is transfered to the calling function

   */