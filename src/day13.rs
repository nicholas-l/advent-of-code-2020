use std::io::BufRead;

pub fn star_one(input: impl BufRead) -> usize {
    let mut lines = input.lines();
    let start = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let ids: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .filter(|x| x != &"x")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut closest = usize::MAX;
    let mut closest_id = 0;
    for id in ids {
        let next_time = ((start as f64 / id as f64).ceil() * id as f64) as usize;
        if next_time < closest {
            closest_id = id;
            closest = next_time;
        }
    }
    closest_id * (closest - start)
}

fn modinv(a: isize, m: isize) -> Option<isize> {
    let a = a % m;
    (0..m).find(|x| (a * x) % m == 1)
}

pub fn star_two(input: impl BufRead) -> usize {
    let ids: Vec<(isize, isize)> = input
        .lines()
        .nth(1)
        .unwrap()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_i, x)| x != &"x")
        .map(|(i, x)| (i as isize, x.parse::<isize>().unwrap()))
        .collect();
    let n_product: isize = ids.iter().map(|x| x.1).product();
    let sum = ids
        .iter()
        .map(|x| {
            /*
            Why we do bus_id (.1) - position (.0) => (a, n)
            0, 7  => (x + 0) ≡ 0 (mod 7)  => x ≡  0 (mod 7)  (2.) => x ≡  7 - 0 (mod 7)  =>  7 - 0, 7
            1, 13 => (x + 1) ≡ 0 (mod 13) => x ≡ -1 (mod 13) (2.) => x ≡ 13 - 1 (mod 13) => 13 - 1, 13
            4, 59 => (x + 4) ≡ 0 (mod 59) => x ≡ -4 (mod 59) (2.) => x ≡ 59 - 4 (mod 59) => 59 - 4, 59
            */
            let n = x.1;
            let a = x.1 - x.0;
            let n_product_n = n_product / n;
            modinv(n_product_n, n).map(|s| a * s * n_product_n).unwrap()
        })
        .sum::<isize>();
    (sum % n_product) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_modinv() {
        assert_eq!(modinv(3, 26), Some(9));
    }

    #[test]
    fn test_star_one() {
        let input = b"939
7,13,x,x,59,x,31,19";
        assert_eq!(star_one(Cursor::new(input)), 295);
    }

    #[test]
    fn test_star_two() {
        let input = b"939
7,13,x,x,59,x,31,19";
        assert_eq!(star_two(Cursor::new(input)), 1068781);
    }

    #[test]
    fn test_star_two_a() {
        let input = b"939
17,x,13,19";
        assert_eq!(star_two(Cursor::new(input)), 3417);
    }
    #[test]
    fn test_star_two_b() {
        let input = b"939
67,7,59,61";
        assert_eq!(star_two(Cursor::new(input)), 754018);
        let input = b"939
67,x,7,59,61";
        assert_eq!(star_two(Cursor::new(input)), 779210);

        let input = b"939
67,7,x,59,61";
        assert_eq!(star_two(Cursor::new(input)), 1261476);

        let input = b"939
1789,37,47,1889";
        assert_eq!(star_two(Cursor::new(input)), 1202161486);
    }
}
