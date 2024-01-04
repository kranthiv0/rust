fn main() {
    //*immuatblity */
    let x=7;
    println!("The number is {x}");
  //*   x=25; */ cant mutate the x
    println!("number not updated number is {x}");
  //* mutable */
    let mut y =20;
    println!("Orginal num {y}");
    y=45;
    println!("mutated num {y}");
    constant();
    shadow();
 }
 //*constnat checks */
 fn constant(){
     const THREE_HORS_IN_SECONDS:u32 = 60*60*3; // cant have mut 
     println!("constant value is {THREE_HORS_IN_SECONDS}")
 }
 //*Shadowing */
 //? which means shadowing another varriable by creating new
 //mut and shadowing does'nt means same.
 fn shadow(){
     let x=" number x is shadowed with STRING";
 println!("{x}");
 }
 
