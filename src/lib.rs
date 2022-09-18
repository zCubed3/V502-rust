pub mod types;
pub mod vm;

#[cfg(test)]
mod tests {
    use crate::v502_make_word;

    #[test]
    fn test_lib() {
        unsafe {
            let w = v502_make_word(1u8, 0);
            assert_eq!(w, 256);
        }
    }
}