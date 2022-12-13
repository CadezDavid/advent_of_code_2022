use crate::Solution;
use std::collections::VecDeque;
pub fn solve(input: &str) -> Solution {
    let mut area: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();

    let w = input.chars().position(|x| x == '\n').unwrap() as isize;
    let h = input.chars().filter(|x| *x == '\n').count() as isize;

    let start = area.iter().position(|c| *c == 'S').unwrap();
    let end = area.iter().position(|c| *c == 'E').unwrap();
    area[start] = 'a';
    area[end] = 'z';

    let directions: Vec<isize> = vec![-w, w, -1, 1];

    fn bfs(graph: &Vec<Vec<usize>>, start: usize, end: Vec<usize>) -> usize {
        let mut todo = VecDeque::new();
        let mut dist: Vec<Option<usize>> = vec![None; graph.len()];
        todo.push_back(start);
        dist[start] = Some(0);
        let mut curr = start;
        while !todo.is_empty() {
            curr = todo.pop_front().unwrap();
            if end.contains(&curr) {
                break;
            }
            for n in graph[curr].iter() {
                if dist[*n] == None {
                    todo.push_back(*n);
                    dist[*n] = Some(dist[curr].unwrap() + 1);
                }
            }
        }
        dist[curr].unwrap()
    }

    let graph1 = (0..area.len())
        .map(|i| {
            directions
                .iter()
                .map(|di| i as isize + *di)
                .filter(|j| {
                    0 <= *j && *j < h * w && area[i] as isize >= area[*j as usize] as isize - 1
                })
                .map(|j| j as usize)
                .collect()
        })
        .collect();

    let graph2 = (0..area.len())
        .map(|i| {
            directions
                .iter()
                .map(|di| i as isize + *di)
                .filter(|j| {
                    0 <= *j && *j < h * w && area[i] as isize <= area[*j as usize] as isize + 1
                })
                .map(|j| j as usize)
                .collect()
        })
        .collect();

    let sum1 = bfs(&graph1, start, vec![end]);
    let sum2 = bfs(
        &graph2,
        end,
        area.iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == 'a' { Some(i) } else { None })
            .collect(),
    );

    Solution::Isize(sum1 as isize, sum2 as isize)
}
