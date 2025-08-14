fn main() {
    fizzbuzz_to(5);
}


fn is_divisiblyby(lhs: u32, rhs: u32) -> bool{
    if rhs == 0{
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> (){
    if is_divisiblyby(n, 15){
        println!("FizzBuzz");
    } else if is_divisiblyby(n,3){
        println!("Fizz");
    } else if is_divisiblyby(n,5){
        println!("Buzz");
    } else {
        println!("{n}");
    }
}

fn fizzbuzz_to(n: u32) -> (){
    for n in 1..=n{
        fizzbuzz(n);
    }

}