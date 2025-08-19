use aoc::days::*;
use rstest::rstest;

#[test]
fn test_read_input() {
    let lines = day02::read_input(day02::INPUT);
}

#[test]
fn test_set_and_game() {
    let oneset = day02::Set {
        red: 3,
        green: 3,
        blue: 3,
    };

    assert_eq!(oneset.is_set_possible(3, 5, 5), true);
    assert_eq!(oneset.is_set_possible(1, 1, 1), false);
    let onegame = day02::Game {
        nr: 1,
        sets: vec![oneset],
    };
    assert_eq!(onegame.is_game_possible(10, 10, 10), true);
}

#[test]
fn test_parse_line() {
    let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
    let game = day02::Game::from_line(line);

    assert_eq!(game.nr, 2);
    assert_eq!(game.sets[0].blue, 1);
    assert_eq!(game.sets[1].red, 1);
    assert_eq!(game.sets[2].green, 1);
    assert_eq!(game.sets[2].blue, 1);
    assert_eq!(game.sets[2].red, 0);

    assert_eq!(game.is_game_possible(10, 10, 10), true);
    assert_eq!(game.is_game_possible(0, 10, 10), false);
    assert_eq!(game.is_game_possible(10, 0, 10), false);
    assert_eq!(game.is_game_possible(10, 10, 0), false);
}

#[test]
fn test_sum_possible_games() {
    assert_eq!(day02::sum_possible_games("resources/day02_sample.txt"), 8);
}

// Part 2
#[test]
fn test_power() {
    let oneset = day02::Set {
        red: 8,
        green: 1,
        blue: 4,
    };

    let anotherset = day02::Set {
        red: 4,
        green: 3,
        blue: 7,
    };

    let onegame = day02::Game {
        nr: 1,
        sets: vec![oneset, anotherset],
    };

    // The power is the product of the minimal amounts of rgb balls needed
    // to make the game possible
    assert_eq!(onegame.power(), 8 * 3 * 7);
}

#[test]
fn test_sum_power() {
    assert_eq!(day02::sum_power("resources/day02_sample.txt"), 2286);
}
