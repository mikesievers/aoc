use aoc::days::*;
use rstest::rstest;

#[test]
fn test_read_input() {
    let lines = day02::read_input(day02::INPUT);
}

#[test]
fn test_parse_line() {
    let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
    let game = day02::Game::from_line(line);

    assert_eq!(game.nr, 2);
    assert_eq!(game.sets[0].blue, 1);
    assert_eq!(game.sets[1].red, 1);
    assert_eq!(game.sets[2].green, 1);

}
