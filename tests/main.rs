#[cfg(test)]
pub fn main() {
    let test = Test {
        _stuff: "jsjsj".to_string(),
    };
    println!("{:?}", test);
}
#[derive(Debug)]
struct Test {
    _stuff: String,
}
