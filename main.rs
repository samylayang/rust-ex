fn main(){
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);

    let _last_val = v.pop();
    println!("_last_val: {:?}", _last_val);
    println!("v: {:?}", v);

    let first = v[0];
    let second = v.get(1);
    println!("first: {:?}", first);
    println!("second: {:?}", second);
}
