use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::aoc_test;

aoc_test!(day: d05, version: main, part1: 5964, part2: 4719);

pub fn solution1(mut line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();

    line_iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .for_each(|l| {
            let (l, r) = l.split_once("|").unwrap();
            map.entry(r.parse().unwrap())
                .or_default()
                .insert(l.parse().unwrap());
        });

    let mut valid_count = 0;
    for ln in line_iter {
        let nums = ln
            .split(",")
            .map(|n| n.parse::<u32>().unwrap())
            .collect_vec();

        if is_valid(&nums, &map) {
            let mid = nums.len() / 2;
            valid_count += nums[mid];
        }
    }

    Ok(valid_count)
}

pub fn solution2(mut line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
    let mut edges: HashSet<(u32, u32)> = HashSet::new();

    line_iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .for_each(|l| {
            let (l, r) = l.split_once("|").unwrap();
            let (l, r): (u32, u32) = (l.parse().unwrap(), r.parse().unwrap());
            edges.insert((l, r));
        });

    let mut count = 0;

    for ln in line_iter {
        let nums = ln
            .split(",")
            .map(|n| n.parse::<u32>().unwrap())
            .collect_vec();

        let hnums: HashSet<u32> = nums.iter().copied().collect();
        let local_edges: HashSet<_> = edges
            .iter()
            .filter(|(l, r)| hnums.contains(l) && hnums.contains(r))
            .copied()
            .collect();

        let (parents, children) = build_dependency_maps(&local_edges);

        if is_valid(&nums, &parents) {
            continue;
        }

        let sorted = topological_sort(parents, children);

        let mid = sorted.len() / 2;
        count += sorted[mid];
    }

    Ok(count)
}

pub fn topological_sort(
    mut parents: HashMap<u32, HashSet<u32>>,
    mut children: HashMap<u32, HashSet<u32>>,
) -> Vec<u32> {
    let mut no_parent_set = parents
        .iter()
        .filter_map(|(k, v)| v.is_empty().then_some(k))
        .copied()
        .collect_vec();

    let mut ordering: Vec<u32> = vec![];

    while let Some(node) = no_parent_set.pop() {
        ordering.push(node);

        let children = children.remove(&node).unwrap_or_default();
        for child in children {
            let parents = parents.entry(child).or_default();
            parents.remove(&node);
            if parents.is_empty() {
                no_parent_set.push(child);
            }
        }
    }

    ordering
}

pub fn is_valid(nums: &[u32], incoming: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut exclude: HashSet<u32> = HashSet::new();

    for n in nums {
        if exclude.contains(n) {
            return false;
        }
        let Some(r#in) = incoming.get(n) else {
            continue;
        };
        exclude.extend(r#in);
    }

    true
}

pub fn build_dependency_maps(
    edges: &HashSet<(u32, u32)>,
) -> (HashMap<u32, HashSet<u32>>, HashMap<u32, HashSet<u32>>) {
    let (parents, children): (HashMap<u32, HashSet<u32>>, HashMap<u32, HashSet<u32>>) =
        edges.iter().fold(
            (HashMap::new(), HashMap::new()),
            |(mut parents, mut children), &(l, r)| {
                children.entry(l).or_default().insert(r);
                parents.entry(r).or_default().insert(l);
                parents.entry(l).or_default();
                children.entry(r).or_default();
                (parents, children)
            },
        );

    (parents, children)
}
