fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn first_test() {
        assert_eq!(2, 2);
    }
}

