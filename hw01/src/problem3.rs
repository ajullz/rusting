/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut p = 2u32;
    let mut flags = vec![false; n as usize];
    let mut prime_nums = Vec::new();

    while p < n {
        if flags[p as usize] == false {
            
            prime_nums.push(p);
            let mut multiplier = 2;
            let mut multiple = multiplier * p;

            while multiple < n {
                flags[multiple as usize] = true;
                multiplier += 1;
                multiple = multiplier * p;
            }
        }
        p += 1;
    }
    prime_nums
}