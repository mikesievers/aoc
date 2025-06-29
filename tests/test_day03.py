from app.day03.day03 import add_mults, execute_tokens, extract_muls, extract_tokens


def test_extract_muls():
    SAMPLE_ROW = (
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    )
    EXPECTED_MULTIPLICANDS = [(2, 4), (5, 5), (11, 8), (8, 5)]
    assert extract_muls(SAMPLE_ROW) == EXPECTED_MULTIPLICANDS


def test_add_mults():
    assert add_mults([(2, 2), (3, 4)]) == 16


def test_final_mults():
    with open("src/app/day03/input.txt", "r") as f:
        text = f.read()
    assert add_mults(extract_muls(text)) == 169021493


EXPECTED_TOKENS_SAMPLE_WITH_SWITCH = [
    (1, "mul", 2, 4),
    (20, "dont", None, None),
    (28, "mul", 5, 5),
    (48, "mul", 11, 8),
    (59, "do", None, None),
    (64, "mul", 8, 5),
]


def test_extract_tokens():
    SAMPLE_WITH_SWITCH = (
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    )

    assert extract_tokens(SAMPLE_WITH_SWITCH) == EXPECTED_TOKENS_SAMPLE_WITH_SWITCH


def test_execute_tokens():
    assert execute_tokens(EXPECTED_TOKENS_SAMPLE_WITH_SWITCH) == 2 * 4 + 8 * 5


def test_final_mults_with_state():
    with open("src/app/day03/input.txt", "r") as f:
        text = f.read()
    assert execute_tokens(extract_tokens(text)) == 111762583
