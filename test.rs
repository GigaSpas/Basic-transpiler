fn main(){
    let a = 0;
    let b = 2;
    let c = 5;
    for i in 0..c{
        a += b;
    }
    if a > c {
        println!("a > c");
    }
    else {println!("a < c");}
}
