import tqdm

with open("puzzle_input.txt", "r") as f:
    puzzle_inpt = [int(line.replace("\n", "")) for line in f.readlines()]

increased = 0

for idx, item in tqdm.tqdm(enumerate(puzzle_inpt[1:])):
    prev_item = puzzle_inpt[idx]
    if item > prev_item:
        increased += 1

print(f"Total increased: {increased}")
