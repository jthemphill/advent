import re
import sys

import z3


def parse_button(button_str: str) -> list[int]:
    assert button_str[0] == "("
    assert button_str[-1] == ")"
    return [int(n) for n in button_str[1:-1].split(",")]


def parse_joltage_goals(joltage_goal: str) -> list[int]:
    assert joltage_goal[0] == "{"
    assert joltage_goal[-1] == "}"
    return [int(n) for n in joltage_goal[1:-1].split(",")]


def solve(indicators: str, buttons: list[list[int]], joltage_goals: list[int]) -> int:
    o = z3.Optimize()

    num_presses = [z3.Int(f"press_{i}") for i in range(len(buttons))]
    for presses in num_presses:
        o.add(presses >= 0)

    for j, joltage_goal in enumerate(joltage_goals):
        # Which button presses affect the current joltage
        relevant_presses = []
        for presses, button in zip(num_presses, buttons):
            # This button is wired up to the joltage we're looking at, so
            # record the number of times we press it
            for n in button:
                if n == j:
                    relevant_presses.append(presses)
        o.add(sum(relevant_presses) == joltage_goal)

    min_presses = o.minimize(sum(num_presses))
    o.check()
    return min_presses.value().as_long()


def main():
    press_sum = 0
    for line in sys.stdin:
        parts = line.split()

        indicators = parts[0]
        buttons = [parse_button(button_str) for button_str in parts[1:-1]]
        joltage_goals = parse_joltage_goals(parts[-1])
        num_presses = solve(indicators, buttons, joltage_goals)
        print(f"Solution for {indicators} {buttons} {joltage_goals}: {num_presses}")
        press_sum += num_presses
    print(f"Total presses: {press_sum}")


if __name__ == "__main__":
    main()
