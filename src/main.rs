fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn simple_assert() {
        assert_eq!(1, 1);
    }
}
