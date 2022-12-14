use crate::Solution;
use std::cmp::Ordering::{self, Equal, Less};
use std::iter::zip;
use itertools::Itertools;
use serde_json::Value;
pub fn solve(input: &str) -> Solution {
    #[derive(Debug, Clone)]
    enum Packet {
        List(Vec<Packet>),
        El(isize),
    }
    impl TryFrom<Value> for Packet {
        type Error = String;
        fn try_from(value: Value) -> Result<Self, Self::Error> {
            match value {
                Value::Number(num) => Ok(El(num.as_u64().unwrap() as isize)),
                Value::Array(arr) => Ok(List(arr.iter().map(|v| Packet::try_from(v.clone()).unwrap()).collect())),
                _ => Err("Invalid Packet".to_owned()),
            }
        }
    }

    use Packet::{El, List};
    fn leq(left: &Packet, right: &Packet) -> Ordering {
        match (left, right) {
            (List(l), List(r)) => {
                match zip(l, r).map(|(x, y)| leq(&x, &y)).find(|x| *x != Equal) {
                    Some(x) => x,
                    None => l.len().cmp(&r.len()),
                }
            }
            (El(l), El(r)) => l.cmp(&r),
            (El(_), List(_)) => leq(&List(vec![left.clone()]), right),
            (List(_), El(_)) => leq(left, &List(vec![right.clone()])),
        }
    }
    // fn to_packet(s: &str) -> Packet {
    //     match s.chars().next() {
    //         Some('[') => {
    //             let mut p = Vec::new();
    //             let mut i = 1;
    //             let mut tmp = String::from("");
    //             for c in s.chars().skip(1) {
    //                 if c == '[' {
    //                     i += 1;
    //                     tmp.push(c);
    //                 } else if c == ']' {
    //                     i -= 1;
    //                     if i == 0 {
    //                         p.push(to_packet(&tmp));}
    //                     tmp.push(c);
    //                 } else if c == ',' && i == 1 {
    //                     p.push(to_packet(&tmp));
    //                     tmp = String::new();
    //                 } else {
    //                     tmp.push(c);
    //                 }
    //             }
    //             List(p)
    //         }
    //         Some(_) => El(s.parse().unwrap()),
    //         None => List(vec![]),
    //     }
    // }
    let sum1 = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| Packet::try_from(serde_json::from_str::<Value>(line).unwrap()).unwrap())
        .tuples::<(_, _)>()
        .map(|(x, y)| leq(&x, &y))
        .enumerate()
        .fold(0, |n, (i, x)| if x == Less {n+i+1} else { n});
    Solution::Isize(sum1 as isize, 0)
}
