pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let is_factor = |x: u32, y: u32| if x == 0 { false } else { y % x == 0};

    (0..limit).fold(0, |acc, n| acc + {
        if factors.iter().any(|&f| is_factor(f, n)) {
            n
        } else {
            0
        }
    })
}
