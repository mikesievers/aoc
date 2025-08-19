use aoc::days::*;
use rstest::rstest;

#[test]
fn test_read_input() {
    let lines = day02::read_input(day02::INPUT);
}

#[test]
fn test_set_and_game(){
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
