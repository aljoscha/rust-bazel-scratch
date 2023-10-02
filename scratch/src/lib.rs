/// Try me!
pub fn a_function() -> u8 {
    42
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_fail() {
        panic!("a failing test");
    }
}
