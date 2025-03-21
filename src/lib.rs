mod libfoo_sys;

pub fn sum(x: i32, y: i32) -> i32 {
    unsafe { libfoo_sys::sum(x, y) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }
}
