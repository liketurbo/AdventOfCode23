use std::{collections::HashMap, thread::current};

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input-1.txt")?;

    let mut cards: Vec<_> = vec![];

    input.split('\n').for_each(|l| {
        if let [card_num, rest] = l.split(':').collect::<Vec<&str>>().as_slice() {
            if let [winning, having] = rest.split('|').collect::<Vec<&str>>().as_slice() {
                cards.push(Card {
                    num: card_num
                        .split(' ')
                        .filter(|c| *c != "")
                        .last()
                        .map(|c| c.parse::<u32>().expect("invalid card num"))
                        .expect("always card num"),
                    winning: winning
                        .trim()
                        .split(' ')
                        .filter(|c| *c != "")
                        .map(|n| n.parse().expect("no num"))
                        .collect::<Vec<_>>(),
                    current: having
                        .trim()
                        .split(' ')
                        .filter(|c| *c != "")
                        .map(|n| n.parse().expect("no num"))
                        .collect::<Vec<_>>(),
                });
            } else {
                panic!("| splitting went wrong");
            }
        } else {
            panic!(": splitting went wrong");
        }
    });

    eprintln!("parsed in num input: {:?}", cards);

    let sum: u32 = cards
        .iter()
        .map(|c| {
            let count = (c.winning.iter().filter(|n| c.current.contains(n)).count() as i32) - 1;

            if count < 0 {
                return 0;
            } else {
                (2 as u32).pow((count) as u32)
            }
        })
        .sum();

    eprintln!("sum of points of all the cards: {:?}", sum);

    let mut new_total = 0;

    cards.iter().for_each(|c| {
        new_total += calc_copies(c, &cards);
    });

    eprintln!("new rules total: {}", new_total);

    Ok(())
}

fn calc_copies(card: &Card, cards: &Vec<Card>) -> u32 {
    let count = card
        .winning
        .iter()
        .filter(|n| card.current.contains(n))
        .count() as u32;

    let mut total = 1;

    //                      1               1     4
    for sub_id in card.num + 1..=(card.num + count) {
        if let Some(sub_card) = cards.iter().find(|c| c.num == sub_id) {
            total += calc_copies(sub_card, cards)
        }
    }

    total
}

#[derive(Debug)]
struct Card {
    num: u32,
    winning: Vec<u32>,
    current: Vec<u32>,
}
