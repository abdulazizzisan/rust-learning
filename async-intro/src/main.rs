#![allow(unused_variables)]


fn main() {
    let wow1 = "wow";
    let wow2 = "☽";
    
    println!("{} == {}", wow1.len(), wow2);
    
    let num1 = hundred();
    let num2 = thousand();
    let num3 = ten();
    let res = smol::block_on( async {
        futures::join!(num1, num2, num3)
        }
    );

    println!("res: {:?}", res);
}

async fn hundred() -> u8 {
    println!("This is hundred..... 100");
    100
}

async fn thousand() -> u32 {
    println!("This is thousand..... 1000");
    1000
}

async fn ten() -> u8 {
    println!("This is ten..... 10");
    10
}