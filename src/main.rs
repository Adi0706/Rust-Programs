// fn main() {
//     let  x= 4 ; //implicit--> compiler will know whats the data type
    // println!("x is :{}",x) ; // {}-> embedds the value of x in the string
    // rust variables are immutable by default so i cannot change the value of x 
    //we need to put a mut keyword to make it mutable
// x = 5 ; 
// t o get rid of the mutablity issue redefine x 
// let x = 5 ; 

// println!("the new value of x is : {}",x) ; 

// const SECONDS_IN_MINS:u32= 500 ; 
// println!("const  value is {}",SECONDS_IN_MINS) ; 

// }
// we can access the same name variables from a different scope to its scope ,but i wont change the value in its original scope.


fn main (){
    let  x = 5  ; 
    println!("x is : {}",x) ; 

    x= 10 ; 
    println!("x is :{}",x) ; 
}


