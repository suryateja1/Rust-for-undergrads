use std::{i8,i16,u8};
use std::io::stdin;
fn main(){
let mut random="Asif Khan";
let mut count=random.chars().count();
println!("{}",count);
let mut charecters=random.chars();
let mut ind_char=charecters.next();

while(count<=random.chars().count()){
    match ind_char{
    Some(x)=>println!("{}",x),
    None => break,
}
ind_char=charecters.next();
count-=1;
}
}

// PRORAM WRITTEN TO GIVE CHAR COUNT AND PRINT CHARS
