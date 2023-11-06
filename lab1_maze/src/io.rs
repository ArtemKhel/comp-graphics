use std::str::FromStr;

pub fn read_n_and_check<T: FromStr, F>(n: usize, check: F, msg: &str) -> Option<Vec<T>>
where
    F: FnMut(&T) -> bool,
    F: Copy,
{
    println!("{msg}");
    for line in std::io::stdin().lines().map(|l| l.expect("Failed to read line")) {
        match line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<T>().ok())
            .collect::<Option<Vec<T>>>()
        {
            Some(v) => {
                if v.len() != n {
                    eprintln!("Too many/few arguments")
                } else if !v.iter().all(check) {
                    eprintln!("Check failed")
                } else {
                    return Some(v);
                }
            }
            None => eprintln!("Failed to parse"),
        }
    }
    None
}

pub fn read_n<T: FromStr>(n: usize, msg: &str) -> Option<Vec<T>> {
    read_n_and_check(n, |_| true, msg)
}
