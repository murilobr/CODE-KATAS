// fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
//     let t1 = a + b + c;
//     let t2 = a * b + c;
//     let t3 = a + b * c;
//     let t4 = a * b * c;
//     let t5 = (a + b) * c;
//     let t6 = a * (b + c);

//     let res1 = std::cmp::max(t1, t2);
//     let res2 = std::cmp::max(res1, t3);
//     let res3 = std::cmp::max(res2, t4);
//     let res4 = std::cmp::max(res3, t5);
//     let res5 = std::cmp::max(res4, t6);

//     return res5;
// }

// fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
//     *vec![
//         a * b * c,
//         a * b + c,
//         a + b * c,
//         a * c + b,
//         a * (b + c),
//         (a + b) * c,
//         a + b + c
//     ].iter().max().unwrap()
// }

// fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
//     match (a, b, c) {
//         (1, 1, 1) => 3,
//         (1, 1, _) => (a + b) * c,
//         (1, _, 1) => a + b + c,
//         (_, 1, 1) => a * (b + c),
//         (1, _, _) => (a + b) * c,
//         (_, 1, _) => std::cmp::max((a + b) * c, a * (b + c)),
//         (_, _, 1) => a * (b + c),
//         _ => a * b * c
//     }
// }

fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    *[a + b + c, a * (b + c), (a + b) * c, a * b * c].iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(expressions_matter(2, 1, 2), 6);
        assert_eq!(expressions_matter(1, 1, 1), 3);
        assert_eq!(expressions_matter(2, 2, 4), 16);
        assert_eq!(expressions_matter(3, 3, 3), 27);
        assert_eq!(expressions_matter(2, 1, 1), 4);
        assert_eq!(expressions_matter(1, 2, 3), 9);
        assert_eq!(expressions_matter(1, 3, 1), 5);
        assert_eq!(expressions_matter(2, 2, 2), 8);

        assert_eq!(expressions_matter(5, 1, 3), 20);
        assert_eq!(expressions_matter(3, 5, 7), 105);
        assert_eq!(expressions_matter(5, 6, 1), 35);
        assert_eq!(expressions_matter(1, 6, 1), 8);
        assert_eq!(expressions_matter(2, 6, 1), 14);
        assert_eq!(expressions_matter(6, 7, 1), 48);

        assert_eq!(expressions_matter(2, 10, 3), 60);
        assert_eq!(expressions_matter(1, 8, 3), 27);
        assert_eq!(expressions_matter(9, 7, 2), 126);
        assert_eq!(expressions_matter(1, 1, 10), 20);
        assert_eq!(expressions_matter(9, 1, 1), 18);
        assert_eq!(expressions_matter(10, 5, 6), 300);
        assert_eq!(expressions_matter(1, 10, 1), 12);  
    }
}