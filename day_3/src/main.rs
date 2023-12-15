use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input-1.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let schematic: Vec<_> = input
        .split("\n")
        .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>())
        .collect();

    let nums = {
        let mut arr: Vec<_> = Vec::new();

        schematic.iter().enumerate().for_each(|(y, row)| {
            let mut current_num = String::new();
            row.iter().enumerate().for_each(|(end_x, ch)| {
                if ch.is_numeric() {
                    current_num.push(*ch);
                } else if !current_num.is_empty() {
                    arr.push(NumberElement::from_schematic(
                        y,
                        end_x,
                        current_num.clone(),
                        &schematic,
                    ));
                    current_num = String::new();
                }
            });

            if !current_num.is_empty() {
                arr.push(NumberElement::from_schematic(
                    y,
                    row.len(),
                    current_num.clone(),
                    &schematic,
                ));
            }
        });
        arr
    };

    let sum: i32 = nums
        .iter()
        .filter(|e| e.surrounding_symbols().iter().any(|s| s.value != '.'))
        .map(|e| e.value)
        .sum();
    eprintln!("sum of filtered: {}", sum);

    //                     Gear's position
    let mut gears: HashMap<ElementPosition, (Option<&NumberElement>, Option<&NumberElement>)> =
        HashMap::new();
    nums.iter().for_each(|e| {
        e.surrounding_symbols()
            .iter()
            .filter(|s| s.value == '*')
            .for_each(|g| {
                gears
                    .entry(g.position)
                    .and_modify(|ps| ps.1 = Some(e))
                    .or_insert((Some(e), None));
            });
    });

    eprintln!("gears len: {}", gears.len());

    let mut chains: Vec<HashSet<&NumberElement>> = vec![];
    gears.iter().filter(|p| p.1 .1.is_some()).for_each(|p| {
        let num_1 = p.1 .0.expect("filtered");
        let num_2 = p.1 .1.expect("filtered");

        let mut new_set: HashSet<&NumberElement> = HashSet::new();
        let found_set = chains
            .iter_mut()
            .find(|c| c.contains(num_1) || c.contains(num_2))
            .unwrap_or(&mut new_set);
        found_set.insert(num_1);
        found_set.insert(num_2);

        if !chains.contains(&new_set) {
            chains.push(new_set);
        }
    });

    eprintln!("chains len: {}", chains.len());
    eprintln!("first chain: {:?}", chains[0]);

    let sum: i32 = chains
        .iter()
        .map(|c| {
            let product = c
                .iter()
                .map(|e| e.value)
                .reduce(|acc, v| acc * v)
                .unwrap_or(0);
            product
        })
        .sum();

    println!("gears's sum: {}", sum);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct NumberElement<'a> {
    value: i32,
    pos: ElementPosition,
    len: usize,
    schematic: &'a Vec<Vec<char>>,
}

#[derive(Debug)]
struct SymbolElement {
    value: char,
    position: ElementPosition,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct ElementPosition {
    x: usize,
    y: usize,
}

impl<'a> NumberElement<'a> {
    pub fn from_schematic(
        y: usize,
        end_x: usize,
        value: String,
        schematic: &'a Vec<Vec<char>>,
    ) -> Self {
        let x = end_x - value.len();
        let new_num_el = NumberElement {
            len: value.len(),
            value: value.parse().expect("some crazy num"),
            pos: ElementPosition { x, y },
            schematic: &schematic,
        };

        new_num_el
    }

    pub fn surrounding_symbols(&self) -> Vec<SymbolElement> {
        self.surrounding_indices()
            .iter()
            .map(|p| SymbolElement {
                position: *p,
                value: *self
                    .schematic
                    .get(p.y)
                    .expect("already sanitized")
                    .get(p.x)
                    .expect("already sanitized"),
            })
            .collect()
    }

    fn surrounding_indices(&self) -> HashSet<ElementPosition> {
        let mut indices = HashSet::new();

        // Generating all possibilities
        for l in 0..self.len {
            for i in -1..=1 {
                for j in -1..=1 {
                    let y = self.pos.y as isize + i;
                    let x = (self.pos.x as isize) + (l as isize) + j;

                    if let (Ok(y), Ok(x)) = (usize::try_from(y), usize::try_from(x)) {
                        indices.insert(ElementPosition { y, x });
                    }
                }
            }
        }

        indices = indices
            .into_iter()
            // Remove locations of digits itself
            .filter(|p| {
                if p.y == self.pos.y && p.x >= self.pos.x && p.x < self.pos.x + self.len {
                    return false;
                }
                true
            })
            // Remove length exceeds
            .filter(|p| {
                p.y < self.schematic.len()
                    && p.x < self.schematic.get(0).expect("at least one element").len()
            })
            .collect();

        indices
    }
}
