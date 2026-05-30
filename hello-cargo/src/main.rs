fn main() {
    let mut v: Vec<i32> = vec![1];

    print!("{:?}", v.get(1));

    v.push(2);

    dbg!(&v);
    v.push(4)
}
