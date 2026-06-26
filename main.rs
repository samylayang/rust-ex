fn main(){
    // Ownership
    let v: Vec<i32> = Vec::new();
    // let mut v ...-> Compile error
    v.len();
    println!("{:?}", v);

    let mut v: Vec<i32> = Vec::new();
    // let v: ... ->  Compile error
    v.push(1);
    println!("{:?}", v);

    // Borrowing
    let v: Vec<i32> = Vec::new();
    let v1 = &v; //vi has bowwoed from v
    let v2 = &v; //v2 has also bowwoed from v
    v.len();
    println!("{:?}", v);
    v1.len();
    println!("{:?}", v1);
    v2.len();
    println!("{:?}", v2);

    // Mutable borrowing
    let mut v: Vec<i32> = Vec::new();
    let v1 = &mut v; //1st mutable reference
    // let v2 = &mut v; //2nd mutable reference
    // error:cannot borrow `v` as mutable more than once at a time
    v1.push(1); 
    println!("{:?}", v1);

}
