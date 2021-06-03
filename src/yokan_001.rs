fn solve(m: &isize, diffs: &Vec<isize>) -> isize {
    let mut stack = 0;
    let mut count = 0;

    for (_, diff) in diffs.iter().enumerate() {
        stack += diff;

        if stack >= *m {
            stack = 0;
            count += 1;
        }
    }

    return count;
}

fn main() {
    let input = read_vec::<isize>();
    let _n = input.get(0).unwrap(); // N個の切れ目
    let l = input.get(1).unwrap(); // 長さL[cm]の羊羹

    let k = read::<isize>(); // K個の切れ目を選ぶ
    let xs = read_vec::<isize>(); // N個の切れ目の位置[cm]

    let mut diffs: Vec<isize> = vec![];
    for (i, x) in xs.iter().enumerate() {
        let diff: isize;
        if i == 0 {
            diff = x.clone()
        } else {
            diff = x - xs.get(i - 1).unwrap();
        }

        diffs.push(diff);

        if i == xs.len() - 1 {
            let last_diff = l - x;
            diffs.push(last_diff);
        }
    }

    let mut left: isize = 0;
    let mut right: isize = l.clone();

    while (right - left) > 1 {
        let mid: isize = left + ((right - left) / 2);

        if solve(&mid, &diffs) > k {
            left = mid
        } else {
            right = mid
        }
    }
    println!("{}", left);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

// fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
//     (0..n).map(|_| read_vec()).collect()
// }
//

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        let vec = vec![1, 2, 3, 4];
        let m = 3;

        let result = solve(&m, &vec);

        assert_eq!(3, result);

        let vec2 = vec![4, 8, 10, 2, 3];
        let m2 = 10;
        let result2 = solve(&m2, &vec2);

        assert_eq!(2, result2);
    }
}
