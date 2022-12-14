use crate::Solution;
use itertools::Itertools;
use serde_json::{from_str, Value};
use std::cmp::Ordering::{self, Equal, Less};
use std::iter::zip;
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
                Value::Array(arr) => Ok(List(
                    arr.iter()
                        .map(|v| Self::try_from(v.clone()).unwrap())
                        .collect(),
                )),
                _ => Err("Invalid Packet".to_owned()),
            }
        }
    }

    use Packet::{El, List};
    fn leq(left: &Packet, right: &Packet) -> Ordering {
        match (left, right) {
            (List(l), List(r)) => zip(l, r)
                .map(|(x, y)| leq(&x, &y))
                .find(|x| *x != Equal)
                .unwrap_or(l.len().cmp(&r.len())),
            (El(l), El(r)) => l.cmp(&r),
            (El(_), List(_)) => leq(&List(vec![left.clone()]), right),
            (List(_), El(_)) => leq(left, &List(vec![right.clone()])),
        }
    }
    let sum1 = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| Packet::try_from(from_str::<Value>(line).unwrap()).unwrap())
        .tuples::<(_, _)>()
        .enumerate()
        .fold(
            0,
            |n, (i, (x, y))| if leq(&x, &y) == Less { n + i + 1 } else { n },
        );

    let div1 = Packet::try_from(from_str::<Value>("[[2]]").unwrap()).unwrap();
    let div2 = Packet::try_from(from_str::<Value>("[[6]]").unwrap()).unwrap();

    let (s1, s2): (isize, isize) = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| Packet::try_from(from_str::<Value>(line).unwrap()).unwrap())
        .fold((1, 2), |(s1, s2), p| {
            match (leq(&p, &div1), leq(&p, &div2)) {
                (Less, _) => (s1 + 1, s2 + 1),
                (_, Less) => (s1, s2 + 1),
                _ => (s1, s2),
            }
        });
    Solution::Isize(sum1 as isize, s1 * s2)
}
