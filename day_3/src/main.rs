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
    eprintln!("schematic of the input:\n{:?}", schematic);

    let nums = {
        let mut arr: Vec<(i32, (usize, usize, usize))> = Vec::new();
        schematic.iter().enumerate().for_each(|(y, row)| {
            let mut current_num = String::new();
            row.iter().enumerate().for_each(|(x, ch)| {
                if ch.is_numeric() {
                    current_num.push(*ch);
                } else if !current_num.is_empty() {
                    arr.push((
                        current_num.parse().expect("some crazy num"),
                        (y, x - current_num.len(), current_num.len()),
                    ));
                    current_num = String::new();
                }
            });

            if !current_num.is_empty() {
                arr.push((
                    current_num.parse().expect("some crazy num"),
                    (y, row.len() - current_num.len(), current_num.len()),
                ));
            }
        });
        arr
    };

    eprintln!("nums with metadata:\n{:?}", nums);

    let filtered_nums: Vec<_> = nums
        .iter()
        .filter(|(num, pos)| {
            let indices: Vec<_> = {
                let mut arr = vec![];

                for l in 0..pos.2 {
                    for i in -1..=1 {
                        for j in -1..=1 {
                            arr.push((pos.0 as isize + i, (pos.1 as isize) + (l as isize) + j))
                        }
                    }
                }

                arr.into_iter()
                    .filter(|(y, x)| {
                        if *y == pos.0 as isize
                            && *x >= pos.1 as isize
                            && *x < pos.1 as isize + pos.2 as isize
                        {
                            return false;
                        }
                        true
                    })
                    .collect()
            };

            if *num == 114 {
                eprintln!("indices of 114 (len): {}", indices.len());
                eprintln!("indices of 114: {:?}", indices);
            }

            indices.iter().any(|ind| {
                let i = usize::try_from(ind.0);
                let j = usize::try_from(ind.1);

                if let (Ok(i), Ok(j)) = (i, j) {
                    if let Some(r) = schematic.get(i) {
                        if let Some(ch) = r.get(j) {
                            if *ch != '.' {
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
        })
        .collect();

    eprintln!("filtered nums with metadata:\n{:?}", filtered_nums);
    eprintln!(
        "sum of filtered: {}",
        filtered_nums.iter().map(|(v, _)| v).sum::<i32>()
    );

    Ok(())
}
