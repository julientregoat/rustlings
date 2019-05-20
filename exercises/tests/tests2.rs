// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Scroll down for hints :)

#[cfg(test)]
mod tests {
    use std::ptr::null;

    #[test]
    fn you_can_assert_eq() {
        let a: *const u8 = null();
        let b: *const u8 = null();

        assert_eq!(a, b);
    }
}





























// Like the previous exercise, you don't need to write any code to get this test to compile and
// run. `assert_eq!` is a macro that takes two arguments and compares them. Try giving it two
// values that are equal! Try giving it two arguments that are different! Try giving it two values
// that are of different types! Try switching which argument comes first and which comes second!
