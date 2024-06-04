fn main() {
      // The Stack and the Heap
    // pushing and popping
    // allocating
    //restaurant
    //rule
    /* Each value in Rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
 */




 example_7();

}


fn example_1(){
    fn a(){
        let x = "here";
        let y =23;
        b();
     }
    
     fn b() {
         let x= String::from("aaa");
     }
    
}

fn example_2(){
    let x = 23;
    let y= x; // copy
   
    print!("{}",x);
   
   
    let s1 = String::from("hello");
   
   //  let s2= s1; // transfer or move
   
    let s2= s1.clone(); // transfer or move
   
   
    print!("{}",s1);
   
   
}

fn  example_3() {

    let s= String::from("hello");
    // makes_copy_string(&s);
    // takes_ownership(s);
        
    println!("{}",s);


    // copy int
    let x =5;
    makes_copy(x);
    println!("{}",x);


    let s1 = gives_ownership();
    println!("s1 = {}",s1);

    let s2 = String::from("hello");
    let s3 = take_and_gives_back(s2);
    println!("s3 = {}",s3);


}

fn example_4(){
 
   let s1 = String::from("Hello");

   let len= get_length(&s1);

   println!("The length of '{}' is '{}' ", s1, len);
}

fn example_5(){

    let mut s1= String::from("hello ");
    change(&mut s1);
    println!("{}",s1);
}
fn example_6() {

    let  s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);


    // but 
    let mut s1 = String::from("hello");
    let t1 = &mut s1;
    println!("{}", t1);
    // t1 scop ends so we can use it
    let t2 = &mut s1;
    println!("{}", t2);


    // the rules for references
    // 1. at a give time , you can have either one mutable reference or any number of immutable references.
    // 2. references must always be valid



}

fn example_7() {
    
    let mut s= String::from("hello");
    let s2 = "hello world";
    let word = first_word(s2);
   }
fn example_8() {
    let array= [1,2,3,4,5];
    let aSlice= &array[0..2];
    let bSlice= &array[2..];
    let full= &array[0..];
    
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]    
}
fn change(some_string: &mut String) {
    some_string.push_str("michael");
}

fn get_length(some_string:&String) -> usize{
    some_string.len()
}

fn take_and_gives_back(some_string: String) -> String {
   some_string
}

fn takes_ownership(some_string: String) {
    println!("{}",some_string);
}

fn makes_copy_string(some_string: &String){
    println!("{}",some_string);
}

fn makes_copy(number: i32){
    println!("{}",number);
}


fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

