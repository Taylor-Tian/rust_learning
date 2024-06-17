fn main() {
    let x = 5; // 不可变变量
    let mut y = 10; // 可变变量
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    y = 20;
    println!("The new value of y is: {}", y);
    let int_val:i32 = 100;
    let float_val: f64 = 3.14;
    let bool_val: bool = true;
    let char_val: char = 'a';
    let tuple_val:(i32,f64,char)=(500,6.4,'a');
    let array_val:[i32;5] = [1,2,3,4,5];

    println!("Interger:{}",int_val);
    println!("Float:{}",float_val);
    println!("Boolean:{}",bool_val);
    println!("Character:{}",char_val);
    println!("Tuple:({},{},{})",tuple_val.0,tuple_val.1,tuple_val.2);
    println!("Array:{:?}",array_val);
}
