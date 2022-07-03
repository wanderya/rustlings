// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)


#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        let s1 = "apple".to_string();
        let s2 = "apple".to_string();
        assert_eq!(s1, s2);
    }
}
