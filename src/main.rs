use std::f64::consts::PI;
fn main() {
    println!("implicit equation:\nxy - y^2 = 1 \nx^2y + y = 5 \nexplicit equation:\nx = 1/y + y\nx = sqrt(5/y -1) ")
}

//simple iteration method
fn iter_method(){
    let mut x = 0f64;
    let mut y = 0f64;
    x = (1f64 + y*y)/y;
    y = 5f64 - x*x*y;

   


}