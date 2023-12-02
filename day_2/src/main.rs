/* Part one
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green | OK
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue | OK
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red | X
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red | X
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green | OK
 */

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() {
    let input = std::fs::read_to_string("day_2/input-1.txt").unwrap();
    let games: Vec<_> = input
        .lines()
        .map(|l| {
            let mut v = Vec::with_capacity(2);
            v.extend(l.split(':'));
            if let [raw_game, raw_sets] = v.as_slice() {
                let sets: Vec<_> = raw_sets
                    .split(';')
                    .map(|s| {
                        let mut r = 0;
                        let mut g = 0;
                        let mut b = 0;

                        s.split(',').for_each(|c| {
                            let mut i = c.trim().split(' ');

                            let amount: i32 = i
                                .next()
                                .expect("always 'n color'")
                                .parse()
                                .expect("valid input set num");

                            let color = i.next().expect("always 'n color'");
                            if color == "red" {
                                r += amount;
                            } else if color == "green" {
                                g += amount;
                            } else if color == "blue" {
                                b += amount;
                            }
                        });

                        (r, g, b)
                    })
                    .collect();

                let game: i32 = raw_game
                    .split(' ')
                    .last()
                    .expect("always 'Game n'")
                    .parse()
                    .expect("valid input game num");

                Game { id: game, sets }
            } else {
                unreachable!()
            }
        })
        .collect();

    let sum_of_games: i32 = games
        .iter()
        .filter(|g| {
            let pass = g
                .sets
                .iter()
                .all(|s| s.0 <= MAX_RED && s.1 <= MAX_GREEN && s.2 <= MAX_BLUE);
            pass
        })
        .map(|g| g.id)
        .sum();

    println!("Sum of those games: {}", sum_of_games);

    use std::cmp::max;

    let sum_of_powers: i32 = games
        .iter()
        .map(|g| {
            let min_amount = g.sets.iter().fold((0, 0, 0), |acc, x| {
                (max(acc.0, x.0), max(acc.1, x.1), max(acc.2, x.2))
            });
            min_amount
        })
        .map(|m| m.0 * m.1 * m.2)
        .sum();

    println!("Sum of powers: {}", sum_of_powers);
}

#[derive(Debug)]
struct Game {
    pub id: i32,
    pub sets: Vec<(i32, i32, i32)>,
}
