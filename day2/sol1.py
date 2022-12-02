import re

inp = open("input.txt", "r")
currentLines = []

gamePoints = {
    "X": {"A": 1+3, "B": 1+0, "C": 1+6},
    "Y": {"A": 2+6, "B": 2+3, "C": 2+0},
    "Z": {"A": 3+0, "B": 3+6, "C": 3+3}
}

total = 0

for line in inp:
    moves = line.strip().split(" ")
    total += gamePoints[moves[1]][moves[0]]
    
print(total)