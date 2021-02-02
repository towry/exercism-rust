pub fn nth(n: u32) -> u32 {
    let mut arr: Vec<u32> = Vec::new();

    let mut i: u32 = 0;
    while arr.len() <= (n + 1) as usize {
        if is_prime(i) {
            arr.push(i)
        }
        i += 1;
    }

    if n as usize >= arr.len() {
        return 0;
    }

    return arr[(n) as usize];
}

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    for p in 2..n {
        if n % p == 0 {
            return false;
        }
    }
    return true;
}
