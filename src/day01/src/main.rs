fn main() {
    let input = include_str!("input.txt")
        .split("\n")
        .map(|f| f.split("").filter(|&a| a.parse::<i32>().is_ok()).collect())
        .filter(|f:&Vec<&str>| f.len() != 0).map(|f| {
        if f.len() == 1  { return format!("{}{}", f[0], f[0])};
        return format!("{}{}", f[0], f[f.len()-1]);
    }).map(|f|f.parse::<i32>().unwrap()).sum::<i32>();

    println!("Result: {:?}", input);
    //println!("Result: {:?}", result)
}
