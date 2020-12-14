use crate::day::Day;

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        let temp_old_r = old_r;
        old_r = r;
        r = temp_old_r - quotient * r;
        let temp_old_s = old_s;
        old_s = s;
        s = temp_old_s - quotient * s;
        let temp_old_t = old_t;
        old_t = t;
        t = temp_old_t - quotient * t;
    }

    (old_r, old_t, old_s)
}

pub struct Day13 {}

impl Day for Day13 {
    type InputElement = Option<usize>;
    type Output1 = usize;
    type Output2 = i128;

    fn solve_part1(&self, input: &Vec<Self::InputElement>) -> Self::Output1 {
        let mut input_iter = input.iter()
                                .filter(|x| x.is_some())
                                .map(|x| x.expect("Invalid Input"));
        let curr_time = input_iter.next().expect("Invalid Input");
        let (id, wait) = input_iter.map(|x| (x, x - (curr_time % x)))
                                    .min_by_key(|x| x.1)
                                    .expect("No results");
        id * wait
    }

    fn solve_part2(&self, input: &Vec<Self::InputElement>) -> Self::Output2 {
        input.iter()
                .skip(1)
                .enumerate()
                .filter(|(_, x)| x.is_some())
                .map(|(i,x)| (i as i128, x.unwrap() as i128))
                .map(|(i,x)| ((x - i) % x, x))
                .fold_first(|(a, m), (b, n)| {
                    let (_gcd, r, s) = extended_gcd(m, n);
                    let res = (r*n*a + s*m*b) % (m*n);
                    if res < 0 {
                        (res + m*n, m*n)
                    } else {
                        (res, m*n)
                    }
                }).unwrap().0
    }

    fn parse_input(&self, content: String) -> Vec<Self::InputElement> {
        content.lines().map(|x|
                            x.split(',')
                             .map(|e| e.parse::<usize>()
                                        .ok())
                        ).flatten().collect()
    }
}
