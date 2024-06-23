fn main() {
    let v1: Vec<i32> = vec![1,2,3,4,5,6];
    let v2: Vec<i32> = v1.iter().map(|x| x * x).collect();
    for val in v2 {
        println!("Square {}", val);
    }

    let v3: Vec<i32> = v1.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 2).collect();
    println!("Filer and doubled: {:?}", v3);

}