import tqdm
from string import ascii_uppercase

with open("puzzle_input.txt", "r") as f:
    puzzle_inpt = [int(line.replace("\n", "")) for line in f.readlines()]

window_sums = []

for idx in range(2, len(puzzle_inpt)):
    window = [puzzle_inpt[idx - 2], puzzle_inpt[idx - 1], puzzle_inpt[idx]]
    window_sums.append(sum(window))

increased = 0

for idx, ws in enumerate(window_sums[1:]):
    prev_ws = window_sums[idx]
    if ws > prev_ws:
        increased += 1

print(f"Total increased: {increased}")
