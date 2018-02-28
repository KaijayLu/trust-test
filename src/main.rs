fn main() {
    for _ in 0..5 {
        println!("Hello, world!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
       main(); 
    }
}
