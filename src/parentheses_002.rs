fn main() {
    let n = read::<usize>();

    for i in 0..(1 << n) {
        let mut s: String = "".to_owned();
        for j in (0..n).rev() {
            if i & (1 << j) == 0 {
                s += "(";
            } else {
                s += ")";
            }
        }

        let mut depth: isize = 0;
        for k in s.as_str().chars() {
            if k == '(' {
                depth += 1
            };

            if k == ')' {
                depth -= 1;
            }

            if depth < 0 {
                break;
            };
        }

        if depth == 0 {
            println!("{}", s);
        }
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
