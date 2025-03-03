mod challenges;
// use std::fs;
// use std::env;


fn main(){
    challenges::chal_ten_enums();
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

  /*
  Chapter 7 borrowing

  Borrowing can be achieved by passing references through the use of & to objects instead of the value

  fn main(){
        let x = String::from("brw");
        let x = borrow(&x);
        println!(x)
  }
  
  fn borrow(x: &string) -> usize{
        x.len();
  }

  This now borrows x and thus does not have to return it to send back its ownership

  borrowing but mutable
    fn main(){
        let mut x = String::from("brw");
        let y = borrow(&mut x);
        println!(x)
  }
  
  fn borrow( x:  &mut string) -> usize{
        x.push_str(" sadfasdfa")
        x.len();
  }

  When using a mutable reference, you can only create one reference to it in that scope. 
    Prevents race conditions in threaded scenarios
    &mut var + &var, not allowed because one is multiple. Reading race condition
  

  dangling references

  fn main(){
    let x = make_ref();
    println!(x);
  }  
  
  fn make_ref() -> &String {
    let new_str = String::from("asdf");
    &new_str
  }
    wont work because as soon as the reference to new_str is returned it goes out of scope so the println has no var to print. Use after free error, not allowed.
    Best to just return new_str the var, not the reference.


    Slices
    can slice parts of a string and use them when borrowed

    let x = String::from("one two");
    let y = &x[3..] // takes two
    those indexes are by byte and not by character so keep in mind special characters like emojis will take up multiple bytes
    can't index past end without crashing
    can also slice sarrays

    let x = [1,23,4,56];
    let y : &[i32] = &x[2..];
    without borrow won't compile as array sizes must be known at compile time, same for string slices.

    btw &String is a string, and str is a slice
    slices as function parameters

    &String can act as &str but &str cant act as a &String because &String contains capacity data. 

   */

  /* Chapter 8 standard library and modules
  
    Pretty simple, very close to C++ 
    use std::io;

    fn main(){
        io::stdin().read_line(&mut someString);
    }
  
    for other modules (crates)
    go to cargo.toml and add
    crate_name = "version number"
    under the dependencies thing.
   */

  /* Chapter 9 Input and output

    Command line arguments
        use args function
        returns an iterator over arguments 
        first arg usually file

    example:
    use std::env
    fn main(){
        for (index, argument) in end::args().enumerate(){
        println!("{argument}, {index}")}
    }

    getting a specific arg.
    use nth method

    let arg2 = env::args().nth(2).unwrap(); (0 indexed, so 2nd arg is actually 3rd)
    
    if there arent 3 args then runtime error.
    if (env::args().len() <= 2) to check. 

    good example of parsing command line args; doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html

    Reading from files:

    use std::fs;


    fn main(){
        let contents = fs::read_to_string("Cargo.toml").unwrap(); // error handling later
        println!("contents is {contents}");

    // by line:
        for line in contents.lines(){
            println!("{line}");
        }

    // by character
        let contents = fs::read("Cargo.toml").unwrap(); // returns a vector of the bytes as a Vec<u8>
        println!("{contents:?}");
    }

    use std::path for doing actual file path work. 

    Writing to files

    use std::fs;

    fn main(){
        let mut text = String::new();
        text.push("line 1\n");
        text.push("line 2\n");

        fs::write("text.txt",text); // all in one, can't write in pieces
    }

    Appending:

    use std::fs;
    use std::io::Write; // or stdd::io::prelude::*;
    fn main(){

        let mut file = fs::OpenOptions::new().append("true").open("text2.txt").unwrap();
        file.Write(b"\nWhatever"); // arg is a Vec<u8> or array of u8s. 
        
    }

   */

  /* Chapter 10 Structs
  
    Very similar to C/C++

    struct Item {
        field1: String,
        field2: i32,
        field3: f64
    }
    fn main(){
        let x : Item = Item {
            field1: String::from("field1"),
            field2: 64,
            field3: 2.40
        };
    }
    println!("{x.field1");

    mutability is the same, add a mut keyword

    can add 
    #[derive(Debug)] 
    before struct definition to get
    println({:? structname}) to work with debug printing

    Struct data is on the stack unless explicity stated to be on the heap. 

    Struct update syntax

    let x2 = Item {
        field1 : String::from("x2field").
        ..x
    }
    just copies the rest of the stuff from x.


    let x2 = Item {
        ..x
    } // cant do this if we continue to use x because x2 has taken ownership of the string in x.field1
    // want to use clone() if thats desired. use #[derive(Clone)] as well. 
    
    Struct subroutines
    uses fn 

    struct Item {
        field1: String,
        field2: i32,
        field3: f64
    }

    impl Item {
        fn get_field1(&self) -> &str { // first argument is always pointer to itself.
            &self.field1;
        }
        fn add_field2(&mut self, add : i32){
            self.field2 += add;
        }
    }

    Associated functions
    does not have a &self paramater. Can't ref itsself basically

    impl Item {
        fn new(field1: &str) -> Item{
            Item {
                field1 : String::from(field1),
                field2 : 10,
                field3 : 64.0
            }
        }
    }

    let mut x = Item::new("asdf");


    Tuple structs
    struct Color (u8,u8,u8);
    struct Point (u8,u8,u8);

    fn get_x(p : Point) -> u8{
        p.1
    }

    fn main(){
        let red = Color(256,0,0)
        get_x(red); // fails
    }


    */

    /* Generic types

        Abstract stand-ins for concrete data types or other properties
        can be used with functions, structs, etc
        Uses <T>

        struct Rectange<T,U> {
            width: T,
            height: T
        }
        let x = Rectangle {
            width: 10,
            height: 4.3
        }
        No runtime cost because compiler will replace.

        can be used for methods

        impl<T,U> Rectangle <T,U> {
            fn get_width(&self) -> T{
                &self.width // needs to be an address as we don't know the type yet. Don't want to accidentally transfer ownership.
            }
        }

        impl Rectangle <u8, u8> { // for concrete instances of u8s.
        }


        fn get_biggest<T>(a : T, b : T) -> T {
            if a > b
                return a;
            else 
                return b;
        } // this will fail as it doesnt know if > is possible

        fn get_biggest<T : std::cmp::PartialOrd>(a : T, b : T) -> T { //in the prelude so can also use PartialOrd
            if a > b
                return a;
            else 
                return b;
        } // fixed with traits.


        Box data type
            Smart pointer, has additional functionallity
            ownership of data it has
                because of this will de-alloc on out of scope

        example:

        use std::mem;

        Struct Shuttle {
            name: String,
            crew_size: u8,
            propellant: f64
        }

        fn main() {
            let vehicle = Shuttle {
                name: String::from("Atlantis"),
                crew_size: 7,
                propellant: 80.0
            }

            println!("vehicle size on stack: {mem::size_of_val(&vehicle)}"); // gunna be 40 size of str pointer + u8 + f64

            let boxed_vehicle : Box<Shuttle> = Box::new(vehicle) {
                name: String::from("Atlantis"),
                crew_size: 7,
                propellant: 80.0
            } 
            // vehicle no longer owns the data
            println!("boxed_vehicle size on stack: {mem::size_of_val(&boxed_vehicle)}"); // gunna be pointer sized
            println!("boxed_vehicle size on stack: {mem::size_of_val(&*boxed_vehicle)}"); // gunna be 40 again

            let unboxed : Shuttle = *boxed_vehicle; // data on stack again, ownership passed to unboxed



        }
     */

    /* Chapter 12 traits

        Very similar to interfaces 

        trait MyToString {
            fn toString(&self) -> String;
        }

        impl MyToString for MyStruct {
            fn toString(&self) -> String{
                format!("Name : {self.name} \t Field 1: "{field1}")
            }
        }

        can also do default impl

        trait MyToString {
            fn toString(&self) -> String{
                "Default MyToString"
            }
        }

        impl MyToString for MyStruct{
            // don't have to create a toString func for my struct now, will automatically use the default.
        }

        can derive some basic traits for structs as well with
        #[derive(ParitalEq, PartialOrd, Debug)] // theres more but I forgot what they are

        Trait Bounds:
            General purpose is to stipulate that a certain type contains its given functionality

        example:

        use std::any;
        use std::fmt;

        fn print_type<T: fmt::Display>(item: T){
            println!("{:?item} is a {any::type_name::<T>}")
        }

        fn main(){
            print_type(13);
            print_type(13.0);
            print_type("thirteen");
            print_type([13]); // this won't work as arrays did not implement trait Display
        }

        fn print_type<T: fmt::Debug>(item: T){
            println!("{item} is a {any::type_name::<T>}")
        }

        fn main(){
            print_type(13);
            print_type(13.0);
            print_type("thirteen");
            print_type([13]); // this will work as they all impl debug.
        }

        multiple trait bounds
        use std::fmt;
        use std::;

        fm compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a : T, b: U){
            if a == T::from(b){ // need U to have copy Trait as ownership gets passed by this T::from(b) call
                println!("{a} is equal to {b}");
            }
            else {
                println!("{a} is not equal to {b}")
            }
        }

        can also do this for easier reading. 
        fm compare_and_print<T , U>(a : T, b: U){
            where T: fmt::Display + PartialEq + From<U>,
                U: fmt::Display + PartialEq + Copy
            if a == T::from(b){
                println!("{a} is equal to {b}");
            }
            else {
                println!("{a} is not equal to {b}")
            }
        }


        Traits on returns

        use std::fmt;

        fn get_displayable(choice: bool){} -> impl fmt::Display{
            if (choice){
                13
            }
            else {
                "13"
            }
        }

        fn main(){
            println!("{get_displayable}") // this won't work as returns must be the same. Compiler can't do it because it wants to replace with concrete impl.
        }


     */

    /*
    Chapter 13, lifetimes

    fn main(){
        let proppellant;
        {
            let rp1 = String::from("RP-1");
            propellant = &rp1; // borrowing rp1 here
            println!("propellant is {propellant}");
        }
        println!("propellant is {propellant}"); // this fails because rp1 goes out of scope and was de-alloced
    }

    fn main(){
        let proppellant;
        let rp1 = String::from("RP-1");
        {
            propellant = &rp1; // borrowing rp1 here
            println!("propellant is {propellant}");
        }
        println!("propellant is {propellant}"); // this is fine as rp1 is still in scope and lifetime
    }


    Manually defining lifetimes 
    fn best_fuel( x: &str, y : &str) -> &str {
        if (x.len() > y.len()){
            return x;
        }
        y;
    }

    fn main(){
        let result;
        let proppellant1 = String::from("RP-12");
        let propellant2 = String::from("RP-8");
        result = best_fuel(&propellant1, &propellant2); // best fuel borrows, doesn't take ownership
        println!("result is {result}"); 
        // this will fail as the best_Fuel function needs to explicitly state lifetimes of return values.
    }
    Help message : this function's return type containts a borrowed value, but the signature does not say whether it is borrowed from x or y    

    fn best_fuel<'a>( x: &'a str, y : &'a str) -> &'a str { // these annotations specify the lifetimes as they relate to one another
    // so its saying the two input lifetimes are the same as the outputs lifetime, aka lifetime of x == lifetime of y  == lifetime of return
        if (x.len() > y.len()){
            return x;
        }
        y;
    }

    fn main(){
        let result;
        let proppellant1 = String::from("RP-12");
        let propellant2 = String::from("RP-1");
        result = best_fuel(&propellant1, &propellant2); // best fuel borrows, doesn't take ownership
        println!("result is {result}"); 
        
    }    

    fn main(){
        let result;
        let proppellant1 = String::from("RP-12");
        {
            let propellant2 = String::from("RP-1");
            result = best_fuel(&propellant1, &propellant2); // best fuel borrows, doesn't take ownership
        }
        println!("result is {result}"); 
        // this will fail as the relative lifetimes of the two inputs are not equivalent
    }    


    Multiple lifetime annotations

    fn best_fuel<'a, 'b>( x: &'a str, y : &'b str) -> &'a str {
        if (x.len() > y.len()){
            return x;
        }
        x
    }

    fn main(){
        let result;
        let proppellant1 = String::from("RP-12");
        {
            let propellant2 = String::from("RP-1");
            result = best_fuel(&propellant1, &propellant2); // best fuel borrows, doesn't take ownership
        }
        println!("result is {result}"); 
        // this will be fine
    }

    Lifetime elision rules
        Each elided lifetime in input position becomes a distinct lifetime parameter.

        If there is exactly one input lifetime position (elided or not), that lifetime is assigned to all elided output lifetimes.

        If there are multiple input lifetime positions, but one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes.

        Otherwise, it is an error to elide an output lifetime.

     Struct lifetime annotations

     struct MyStruct {
        name: String
     }

     impl MyStruct {
        fn func(&self, msg: &str) -> &str {
            println!("{msg} {name}");
            self.name
        }
     }

     fn main(){
        let x = MyStrict{
            name: String::from("Name");
        }
        let y = x.func("whatever");
        println!("y is {y}");
     } // this is all good because MyStruct uses a String


     struct MyStruct {
        name: &str
     }

     impl MyStruct {
        fn func(&self, msg: &str) -> &str {
            println!("{msg} {name}");
            self.name
        }
     }

     fn main(){
        let x = MyStrict{
            name: "Name"
        }
        let y = x.func("whatever");
        println!("y is {y}");
     } // this will fail because theres no lifetime on name because its a ref. 


     struct MyStruct<'a> {
        name: &'a str
     }

     impl<'a> MyStruct<'a> {
        fn func(& self, msg: &str) -> &str {
            println!("{msg} {name}");
            self.name
        }
     }

     fn main(){
        let x = MyStrict{
            name: "Name"
        }
        let y = x.func("whatever");
        println!("y is {y}");
     } // This will work because we added annotations


      trying to return msg from the func will fail because lifetime of Struct and return type don't match in elision rule 3. 

      add &'a self, &'b msg and -> &'b str 
      and impl <'a, 'b> Struct<'a>

      'static lifetime means that it is always available. String literals are examples of this. 
      Can be coerced down if required. 
     */

    /*
    Chapter 14 enums

    #[derive(Debug)]
    enum Shape {
        Rectangle,
        Triangle,
        Circle
    }

    fn main(){
        let my_shape = Shape::Rectangle;
        println!("my_shape is {:?my_shape}")
    }

        #[derive(Debug)]
    enum Shape {
        Rectangle(f64,f64),
        Triangle(f64,f64,f64),
        Circle(f64)
    }

    fn main(){
        let my_shape = Shape::Rectangle(1.2, 3.4);
        println!("my_shape is {:?my_shape}")
    }

    enum Command{ // commands for a Draw() functions
        Clear,
        DrawLine(f64,f64),
        DrawShape(Shape)
    }

    match operator
    similar to switch statements

    fn main(){

        match my_shape {
            Shape::Circle(r)
                => circleFunc(r),
            Shape::Rectangle(h,w)
                => rectFunc(h,w),
            Shape::Triangle(s1,s2,s3)
                => triFunc(s1,s2,s3)
        }


    }
    
    fn main() {
        let my_number = 1u8;

        let result = match my_number { // all possible scenarios should be covered
            0 => "zero",
            1 => "one",
            // etc
            _ => { // default case
                println!("num did not match"); 
                "something else"
            } 
        };
    }

    Enum methods
    
    #[derive(Debug)]
    enum Shape {
        Rectangle,
        Triangle,
        Circle
    }

    impl Shape {
        fn get_perimeter(&self) -> f64 {
            match &self {
                Shape::Rectangle => getRectPeri(&self);
                Shape::Circle => getCirclePeri(&self);
                // etc
            }

        }
    }

    Representing nothing
        Rust doesn't have null
    
    enum Option<T>{
        Some(T),
        None
    } // in prelude

    fn main(){
        let countdown = [1,2,3]
        let number = countdown.get(4);
        // will be None
        let other = countdown.get(2);
        // will be Some(3)

        //now can do 
        let b = number.unwrap_or(&0) + 1;

        // or 

        let number = match number {
            Some(num) => num + 1,
            None => 0
        }
    }

    If-let syntax

    fn main(){
        let number = Some(13);

        match number {
            Some(13) => println!("thirteen"),
            _ => ()
        }

        // simpler way to do above
        if let Some(13) = number {
            println!("thirteen")
        }
    }

     */

    /*
    Chapter 15 Runtime errors
    
    Recoverable vs Unrecoverable errors

    Recoverable:
        File not found, just returns an error
        Handled with Result<T,E> enum type
    Unrecoverable:
        Indexing beyond bounds of an array, panics
        Divide by 0
        handled with panic!
        terminates immediately

    Result<T,E> enum

    enum REsult<T,E> {
        Ok<T>,
        Error<E>
    } // in prelude

     

     */