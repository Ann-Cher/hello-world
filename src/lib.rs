fn hello() -> String {
    String::from("hello")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(::hello(), "hello");
    }
}
