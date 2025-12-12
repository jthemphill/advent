import concurrent.futures
import sys
import typing

import z3

# Every present is 3x3
DIM = 3


def rotate_clockwise(present: list[str]) -> list[str]:
    rotated = []
    for r in range(DIM):
        line = ""
        for c in range(DIM):
            line += present[c][-r - 1]
        rotated.append(line)
    return rotated


def reflect_y(present: list[str]) -> list[str]:
    reflected = []
    for r in range(DIM):
        line = ""
        for c in range(DIM):
            line += present[r][-c - 1]
        reflected.append(line)
    return reflected


def orientations(present: list[str]) -> list[list[str]]:
    all_orientations = []
    for _ in range(4):
        all_orientations.append(present)
        present = rotate_clockwise(present)
    present = reflect_y(present)
    for _ in range(4):
        all_orientations.append(present)
        present = rotate_clockwise(present)

    return make_unique(all_orientations)


def make_unique(orientations: list[list[str]]) -> list[list[str]]:
    orientation_set = set()
    for o in orientations:
        orientation_set.add(tuple(o))
    return [list(o) for o in orientation_set]


class Fit:

    width: int
    height: int
    presents_needed: list[int]

    def __init__(self, line):
        split_x = line.split("x")
        self.width = int(split_x[0])
        split_colon = split_x[1].split(": ")
        self.height = int(split_colon[0])

        self.presents_needed = []
        for n in split_colon[1].split():
            self.presents_needed.append(int(n))

    def __repr__(self):
        return f"{self.width}x{self.height}: {' '.join([str(p) for p in self.presents_needed])}"


def present_fits_somewhere(
    grid: list[list[z3.Int]],
    present: list[list[str]],
    present_num: int,
    num_identical_presents_remaining: int,
):
    fits_somewhere = []
    for offset_row in range(len(grid) - DIM + 1):
        for offset_col in range(len(grid[offset_row]) - DIM + 1):
            fits_somewhere.append(
                present_fits_here(
                    grid,
                    present,
                    present_num,
                    offset_row,
                    offset_col,
                    num_identical_presents_remaining,
                )
            )
    return z3.Or(*fits_somewhere)


def present_fits_here(
    grid: list[list[z3.Int]],
    present: list[list[str]],
    present_num: int,
    offset_row: int,
    offset_col: int,
    num_identical_presents_remaining: int,
):
    grid_constraints = []

    # Enforce that no upcoming identical present can be above this present
    # (Performance optimization that makes use of symmetry)
    for r in range(offset_row + 1):
        for c in range(len(grid[r])):
            if r < offset_row or (r == offset_row and c < offset_col):
                for p in range(present_num + 1, num_identical_presents_remaining + 1):
                    grid_constraints.append(grid[r][c] != p)

    for r in range(DIM):
        for c in range(DIM):
            if present[r][c] == "#":
                grid_constraints.append(
                    grid[offset_row + r][offset_col + c] == present_num
                )
    return z3.And(*[grid_constraints])


def solve(presents: list[list[str]], fit: Fit) -> bool:
    # Count the number of #s across all presents
    total_present_sum = 0
    for i, n in enumerate(fit.presents_needed):
        present_sum = 0
        present = presents[i]
        for r in range(DIM):
            for c in range(len(present[r])):
                if present[r][c] == "#":
                    present_sum += 1
        total_present_sum += n * present_sum

    # We can trivially fit all the presents in, just by pretending each present
    # is the width of its block
    if sum(fit.presents_needed) * DIM * DIM <= fit.width * fit.height:
        return True

    # We couldn't fit all the presents in no matter how we arranged them
    if total_present_sum > fit.width * fit.height:
        return False

    s = z3.Solver()

    grid = [
        [z3.Int(f"grid_{r}_{c}") for c in range(fit.width)] for r in range(fit.height)
    ]

    total_num_presents = sum(fit.presents_needed)
    for r in range(fit.height):
        for c in range(fit.width):
            s.add(z3.And(-1 <= grid[r][c], grid[r][c] < total_num_presents))

    present_num = 0
    for i, n in enumerate(fit.presents_needed):
        present = presents[i]
        all_orientations = orientations(present)
        num_spaces = sum(
            [sum([present[r][c] == "#" for c in range(DIM)]) for r in range(DIM)]
        )
        for j in range(n):
            # The present must take up exactly `num_spaces` spaces in grid
            s.add(
                z3.Sum(
                    [
                        z3.Sum(
                            [
                                z3.If(grid[r][c] == present_num, 1, 0)
                                for c in range(len(grid[0]))
                            ]
                        )
                        for r in range(len(grid))
                    ]
                )
                == num_spaces
            )
            # The present's shape must fit in the grid
            s.add(
                z3.Or(
                    [
                        present_fits_somewhere(grid, o, present_num, n - j - 1)
                        for o in all_orientations
                    ]
                )
            )

            present_num += 1
    result = s.check()
    if result == z3.sat:
        print_model(grid, s.model())
    return result == z3.sat


def print_model(grid: list[list[z3.Int]], model: z3.ModelRef):
    s = ""
    for r in range(len(grid)):
        line = ""
        for c in range(len(grid[r])):
            cell = model[grid[r][c]]
            if cell == -1:
                line += "."
            else:
                line += str(cell)
        s += line
        s += "\n"
    print(s)


def main():
    presents = []
    fits = []

    current_present = []
    for line in sys.stdin:
        line = line.rstrip()
        if not line:
            presents.append(current_present)
        elif line.endswith(":"):
            current_present = []
        elif "x" in line:
            fits.append(Fit(line))
        else:
            current_present.append(line)

    for present in presents:
        for o in orientations(present):
            for line in o:
                print(line)
            print()
        print("=====")

    # solutions = [solve(presents, fit) for fit in fits]

    with concurrent.futures.ProcessPoolExecutor() as executor:
        futures = [executor.submit(solve, presents, fit) for fit in fits]
    solutions = [f.result() for f in futures]

    num_sat = 0
    for can_fit in solutions:
        if can_fit:
            num_sat += 1
    print(f"{num_sat} regions can fit")


if __name__ == "__main__":
    main()
