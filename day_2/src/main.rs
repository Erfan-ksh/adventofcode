#[derive(Debug, Eq, PartialEq)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

impl Moves {
    fn value(&self) -> i32 {
        match *self {
            Moves::Rock => 1,
            Moves::Paper => 2,
            Moves::Scissors => 3,
        }
    }
}
#[derive(Debug)]
struct Game {
    my_move: Moves,
    opponent_move: Moves,
}

impl Game {
    fn new(mv: char, om: char) -> Self {
        let opponentmove = match om {
            'A' => Moves::Rock,
            'B' => Moves::Paper,
            'C' => Moves::Scissors,
            not => panic!("unknown move {}", not),
        };
        let mymove = match mv {
            'Y' => Moves::Paper,
            'X' => Moves::Rock,
            'Z' => Moves::Scissors,
            not => panic!("unknown move {}", not),
        };

        Self {
            my_move: mymove,
            opponent_move: opponentmove,
        }
    }
    fn new_with_result(om: char, outcome: char) -> Self {
        if outcome == 'Y' {
            let my_move = match om {
                'A' => 'X',
                'B' => 'Y',
                'C' => 'Z',
                not => panic!("unknown move {}", not),
            };
            return Self::new(my_move, om);
        }
        if outcome == 'X' {
            if om == 'A' {
                return Self::new('Z', om);
            }
            if om == 'B' {
                return Self::new('X', om);
            }
            if om == 'C' {
                return Self::new('Y', om);
            }
        }
        if outcome == 'Z' {
            if om == 'A' {
                return Self::new('Y', om);
            }
            if om == 'B' {
                return Self::new('Z', om);
            }
            if om == 'C' {
                return Self::new('X', om);
            } else {
                panic!("unknown move")
            }
        } else {
            panic!("unknown outcome")
        }
    }
    fn game_result_point(&self) -> i32 {
        let my_move = &self.my_move;
        let opponent_move = &self.opponent_move;
        if my_move == opponent_move {
            return 3;
        }
        if my_move == &Moves::Rock && opponent_move == &Moves::Paper {
            return 0;
        }
        if my_move == &Moves::Rock && opponent_move == &Moves::Scissors {
            return 6;
        }
        if my_move == &Moves::Paper && opponent_move == &Moves::Scissors {
            return 0;
        }
        if my_move == &Moves::Paper && opponent_move == &Moves::Rock {
            return 6;
        }
        if my_move == &Moves::Scissors && opponent_move == &Moves::Rock {
            return 0;
        }
        if my_move == &Moves::Scissors && opponent_move == &Moves::Paper {
            return 6;
        } else {
            panic!("unknown situation")
        }
    }
}

use std::fs;

fn main() {
    let games = fs::read_to_string("data.txt").unwrap();
    let mut all_scores_outcome: Vec<i32> = Vec::new();
    for game in games.lines() {
        let opponent_move = game.chars().nth(0).unwrap();
        let my_move = game.chars().nth(2).unwrap();
        let game_outcome = Game::new_with_result(opponent_move, my_move);
        let sum_game_outcome = game_outcome.game_result_point() + game_outcome.my_move.value();
        all_scores_outcome.push(sum_game_outcome)
    }
    println!("{:?}", all_scores_outcome.iter().sum::<i32>());
}
