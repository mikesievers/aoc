import re

PAT_MUL = r"mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\)"
PAT_DO = r"do\(\)"
PAT_DONT = r"don't\(\)"


def extract_muls(text: str) -> list[(int, int)]:
    """
    Find all instances matching mul([0-9]{1,3},[0-9]{1,3})
    Note:
    * groups are done in re via ()

    emails = re.findall(r'[\w\.-]+@[\w\.-]+', str)

    match = re.search(r'([\w.-]+)@([\w.-]+)', str)
    if match:
        print(match.group())   ## 'alice-b@google.com' (the whole match)
        print(match.group(1))  ## 'alice-b' (the username, group 1)
        print(match.group(2))  ## 'google.com' (the host, group 2)
    """
    pat = r"mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\)"

    multiplicands = []
    matches = re.findall(pat, text)
    for match in matches:
        multiplicands.append((int(match[0]), int(match[1])))

    return multiplicands


def add_mults(multiplicands: list[(int, int)]) -> int:
    return sum([m1 * m2 for (m1, m2) in multiplicands])


def extract_tokens(text: str) -> list[(int, str, int, int)]:
    tokens = []
    for match in re.finditer(PAT_DO, text):
        tokens.append((match.start(), "do", None, None))
    for match in re.finditer(PAT_DONT, text):
        tokens.append((match.start(), "dont", None, None))
    for match in re.finditer(PAT_MUL, text):
        tokens.append((match.start(), "mul", int(match.group(1)), int(match.group(2))))

    tokens.sort(key=lambda x: x[0])
    return tokens


def execute_tokens(tokens: list[(int, str, int, int)]) -> int:
    # Start in active state, i.e. if encountering a mul, execute it
    is_active = True

    sum = 0

    for token in tokens:
        match token[1]:
            case "do":
                is_active = True
            case "dont":
                is_active = False
            case "mul":
                if is_active:
                    sum += token[2] * token[3]

    return sum
