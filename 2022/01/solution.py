
def main():
    part2()

def part1():
    with open('input.txt', 'r') as f:
        bestElf = 0
        currentElf = 0
        for line in f:
            if not line[:-1]:
                bestElf = max(bestElf, currentElf)
                currentElf = 0
            else:
                currentElf += int(line)
        print(bestElf)

def insertElf(bestElfs, elf):
    if elf > bestElfs[0]:
        bestElfs[0] = elf
    bestElfs.sort()

def part2():
    with open('input.txt', 'r') as f:
        bestElfs = [0, 0, 0]
        currentElf = 0
        for line in f:
            if not line[:-1]:
                insertElf(bestElfs, currentElf)
                currentElf = 0
            else:
                currentElf += int(line)
        print(sum(bestElfs))

if __name__ == "__main__":
    main()
