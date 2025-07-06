def read_files(fname: str) -> tuple[list[tuple[int, int]], list[list[int]]]:
    rules = []
    pages = []

    with open(f"src/app/day05/{fname}") as f:
        lines = f.readlines()
        for line in lines:
            rule_elements = line.strip().split("|")
            if len(rule_elements) > 1:
                rules.append((int(rule_elements[0]), int(rule_elements[1])))

            page_elements = line.strip().split(",")
            if len(page_elements) > 1:
                pages.append(list(map(int, page_elements)))

    return rules, pages


def are_rules_observed(rules, pages) -> list[bool]:
    result = []
    for page_seq in pages:
        result.append(check_one_line(rules, page_seq))

    return result


def check_one_line(rules, page_seq):
    for first_page, second_page in rules:
        try:
            if page_seq.index(first_page) > page_seq.index(second_page):
                return False  # One violation is enough
        except ValueError:
            pass  # At least one element of the rule was not in the pages - that's OK

    return True  # If we are here, no violation has been committed


def middle_page_sum(mask, pages):
    middle_pages = [page_seq[len(page_seq) // 2] for page_seq in pages]
    ok_middle_pages = [middle_pages[i] for (i, _) in enumerate(mask) if mask[i] is True]
    return sum(ok_middle_pages)


def make_sequence_correct(rules, incorrect_seq: list[int]) -> list[int]:
    seq = incorrect_seq.copy()
    while not check_one_line(rules, seq):
        seq = switch_first_offender(rules, seq)
    return seq


def switch_first_offender(rules, seq):
    new_seq = seq.copy()
    for first_page, second_page in rules:
        try:
            idx_first = seq.index(first_page)
            idx_second = seq.index(second_page)
            if idx_first > idx_second:
                new_seq[idx_first] = seq[idx_second]
                new_seq[idx_second] = seq[idx_first]
                return new_seq
        except ValueError:
            pass  # At least one element of the rule was not in the pages - that's OK

    return new_seq  # nothing needed changing if we get here
