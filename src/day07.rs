//! --- Day 7: Handy Haversacks ---
//!
//! You land at the regional airport in time for your next flight.
//! In fact, it looks like you'll even have time to grab some food:
//! all flights are currently delayed due to issues in luggage processing.
//!
//! Due to recent aviation regulations, many rules (your puzzle input)
//! are being enforced about bags and their contents; bags must be color-coded
//! and must contain specific quantities of other color-coded bags.
//! Apparently, nobody responsible for these regulations considered how long they
//! would take to enforce!
//!
//! For example, consider the following rules:
//!
//! light red bags contain 1 bright white bag, 2 muted yellow bags.
//! dark orange bags contain 3 bright white bags, 4 muted yellow bags.
//! bright white bags contain 1 shiny gold bag.
//! muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
//! shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
//! dark olive bags contain 3 faded blue bags, 4 dotted black bags.
//! vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
//! faded blue bags contain no other bags.
//! dotted black bags contain no other bags.
//!
//! These rules specify the required contents for 9 bag types. In this example,
//! every faded blue bag is empty, every vibrant plum bag contains 11 bags
//! (5 faded blue and 6 dotted black), and so on.
//!
//! You have a shiny gold bag. If you wanted to carry it in at least one other bag,
//! how many different bag colors would be valid for the outermost bag?
//! (In other words: how many colors can, eventually, contain at least one shiny gold bag?)
//!
//! In the above rules, the following options would be available to you:
//!
//!     A bright white bag, which can hold your shiny gold bag directly.
//!     A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
//!     A dark orange bag, which can hold bright white and muted yellow bags,
//!         either of which could then hold your shiny gold bag.
//!     A light red bag, which can hold bright white and muted yellow bags,
//!         either of which could then hold your shiny gold bag.
//!
//! So, in this example, the number of bag colors that can eventually contain at
//! least one shiny gold bag is 4.
//!
//! How many bag colors can eventually contain at least one shiny gold bag?
//! (The list of rules is quite long; make sure you get all of it.)
use std::collections::{HashMap, HashSet};

pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day07.txt")?;
    println!("Day07 part1: {}", part1(&input));

    Ok(())
}

fn part1<'a>(s: &'a str) -> usize {
    let mut bags = HashMap::<&'a str, Bag<'a>>::new();

    for (name, children) in s
        .trim()
        .lines()
        .map(|line| parse_bag_line(line).expect("parsing bag line"))
    {
        bags.entry(name).or_insert_with(|| Bag::named(name));

        for child in children {
            bags.entry(child.name)
                .or_insert_with(|| Bag::named(child.name))
                .parents
                .push(name);
        }
    }

    let gold_bag = bags.get("shiny gold").expect("que?! no shiny gold bag");

    let mut contains_goldy: Vec<&'a str> = Vec::new();
    rec_diver(&gold_bag.parents, &bags, &mut contains_goldy);

    let unique_containers = contains_goldy.into_iter().collect::<HashSet<_>>();

    unique_containers.len()
}

fn rec_diver<'a>(parents: &[&'a str], bags: &HashMap<&'a str, Bag<'a>>, res: &mut Vec<&'a str>) {
    for par_name in parents {
        res.push(par_name);
        if let Some(parent) = bags.get(par_name) {
            rec_diver(&parent.parents, &bags, res);
        }
    }
}

fn parse_bag_line<'a>(s: &'a str) -> crate::Result<(&'a str, impl Iterator<Item = ChildBag<'a>>)> {
    let mut split = s.trim().split("bags contain");
    let name = split
        .next()
        .ok_or_else(|| format!("initial part missing from bag line `{}`", s))?
        .trim();

    let contains_s = split
        .next()
        .ok_or_else(|| format!("contains part missing from bag line `{}`", s))?
        .trim();

    let contains = contains_s
        .split(',')
        .filter(|s| *s != "no other bags.")
        .map(|child| child.trim())
        .map(|child| {
            let (count_s, rest) = child.split_at(1);
            let count = count_s
                .parse::<usize>()
                .map_err(|_| format!("Invalid count in `{}`", child))
                .expect("Parsing child count");

            let (name, _) = rest.split_at(rest.find("bag").expect("finding `bag`"));
            ChildBag {
                name: name.trim(),
                _count: count,
            }
        });

    Ok((name, contains))
}
#[derive(Debug)]
struct Bag<'a> {
    name: &'a str,
    parents: Vec<&'a str>,
}

impl<'a> Bag<'a> {
    fn named(name: &'a str) -> Self {
        Self {
            name,
            parents: Vec::new(),
        }
    }
}

struct ChildBag<'a> {
    _count: usize,
    name: &'a str,
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_example() {
        let input = r#"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#
        .trim();

        assert_eq!(4, super::part1(input));
    }

    #[test]
    fn part1() {
        let input = crate::read_input("day07.txt").expect("reading input");
        assert_eq!(316, super::part1(&input));
    }
}
