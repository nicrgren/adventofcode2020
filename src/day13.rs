pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day13.txt")?;
    println!("Day13 part1: {}", part1(&input));
    Ok(())
}

fn part1(s: &str) -> u64 {
    let notes = parse(s);
    let mut ttw = 0; // time to wait;

    loop {
        if let Some(bus_id) = notes
            .buses
            .iter()
            .find(|&t| (notes.earliest + ttw) % t == 0)
        {
            return bus_id * ttw;
        }

        ttw += 1;
    }
}

fn parse(s: &str) -> Notes {
    let mut lines = s.trim().lines();
    let earliest = lines
        .next()
        .expect("finding first line")
        .parse::<u64>()
        .expect("parsing earliest time");

    let buses = lines
        .next()
        .expect("finding 2nd line with bus ids")
        .split(',')
        .map(|s| s.trim())
        .filter(|s| *s != "x")
        .map(|s| s.parse::<u64>().expect("parsing bus id"))
        .collect::<Vec<_>>();

    Notes { earliest, buses }
}

struct Notes {
    earliest: u64,
    buses: Vec<u64>,
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_example() {
        let s = r#"
939
7,13,x,x,59,x,31,19
"#;

        assert_eq!(295, super::part1(s));
    }

    #[test]
    fn part1() {
        let input = crate::read_input("day13.txt").expect("reading input");
        assert_eq!(2406, super::part1(&input));
    }
}
