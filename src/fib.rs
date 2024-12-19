use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static GLOBAL_MAP: OnceLock<Mutex<HashMap<u32, i32>>> = OnceLock::new();

pub fn fibonacci(n: u32) -> i32 {
    GLOBAL_MAP.get_or_init(|| {
        let map = HashMap::new();
        Mutex::new(map)
    });
    fibonacci_memo(n)
}

fn fibonacci_memo(n: u32) -> i32 {
    if let Some(&value) = GLOBAL_MAP.get().unwrap().lock().unwrap().get(&n) {
        return value;
    }
    let value = match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_memo(n - 1) + fibonacci_memo(n - 2),
    };

    GLOBAL_MAP.get().unwrap().lock().unwrap().insert(n, value);
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
        assert_eq!(fibonacci(8), 21);
        assert_eq!(fibonacci(9), 34);
        assert_eq!(fibonacci(10), 55);
    }
}
