import re

inp = open("input.txt", "r")
currentLines = []

gamePoints = {
    "A": {"X": 3+0, "Y": 1+3, "Z": 2+6},
    "B": {"X": 1+0, "Y": 2+3, "Z": 3+6},
    "C": {"X": 2+0, "Y": 3+3, "Z": 1+6}
}

total = 0

for line in inp:
    moves = line.strip().split(" ")
    total += gamePoints[moves[0]][moves[1]]
    
print(total)