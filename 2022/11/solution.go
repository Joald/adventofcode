package main

import (
	"fmt"
	"strings"
	"strconv"
	"os"
)

const testStr1 = `Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
	If true: throw to monkey 2
	If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
	If true: throw to monkey 2
	If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
	If true: throw to monkey 1
	If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
	If true: throw to monkey 0
	If false: throw to monkey 1
`;

type Monkey struct {
	num int
	items []int
	op string//func(old int) int
	lop int 
	rop int
	div int
	ifTrue int
	ifFalse int
	inspects int
}

func check(e error) {
	if e != nil {
			panic(e)
	}
}

// // too many allocs to use with big.Ints
// func parseOp(operands []string) func(old int)int {
// 	binOp := func(a int, b int)int { 
// 		return big.NewInt(-1)
// 	}
// 	if operands[1] == "+" {
// 		binOp = func(a int, b int)int {
// 			return a.Add(a, b)
// 		}
// 	} else if operands[1] == "*" {
// 		binOp = func(a int, b int)int {
// 			return a.Mul(a, b)
// 		}
// 	} else {
// 		panic("invalid binary operation " + operands[1])
// 	}
// 	op1 := func(old int)int {
// 		return old
// 	}
// 	op2 := op1
	
// 	if operands[0] != "old" {
// 		v1, err := strconv.Atoi(operands[0])
// 		check(err)
// 		v1b := big.NewInt(int64(v1))
// 		op1 = func(_ int)int {
// 			return v1b
// 		}
// 	}
// 	if operands[2] != "old" {
// 		v2, err := strconv.Atoi(operands[2])
// 		check(err)
// 		v2b := big.NewInt(int64(v2))
// 		op2 = func(_ int)int {
// 			return v2b
// 		}
// 	}

// 	return func(old int)int {
// 		return binOp(op1(old), op2(old))
// 	}
// }

func doOp(monkey Monkey, item int)int {
	var rop int
	if monkey.rop == -1 {
		rop = item
	} else {
		rop = monkey.rop
	}

	if monkey.op == "+" {
		return item + rop
	} else {
		return item * rop
	}
}

func parseOperand(operand string)int {
	if operand == "old" {
		return -1
	}
	op, err := strconv.Atoi(operand)
	check(err)
	return op
}

func parseMonkey(lines []string)Monkey {
	num, err := strconv.Atoi(strings.Split(strings.Split(lines[0], " ")[1], ":")[0])
	check(err)
	
	_, itemString, _ := strings.Cut(lines[1], ": ")
	itemStrings := strings.Split(itemString, ", ")
	items := make([]int, len(itemStrings))
	for i, s := range itemStrings {
		item, err := strconv.Atoi(s)
		check(err)
		items[i] = item
	}
	_, operandString, _ := strings.Cut(lines[2], "= ")
	operands := strings.Split(operandString, " ")
	op := operands[1]
	lop := parseOperand(operands[0])
	rop := parseOperand(operands[2])
	
	_, divStr, _ := strings.Cut(lines[3], "by ")
	div, err := strconv.Atoi(divStr)
	check(err)

	_, ifTrueStr, _ := strings.Cut(lines[4], "monkey ")
	ifTrue, err := strconv.Atoi(ifTrueStr)
	check(err)

	_, ifFalseStr, _ := strings.Cut(lines[5], "monkey ")
	ifFalse, err := strconv.Atoi(ifFalseStr)
	check(err)

	return Monkey{num, items, op, lop, rop, div, ifTrue, ifFalse, 0}
}

func getInput(fromFile bool) []string {
	var str string
	if fromFile {
		dat, err := os.ReadFile("input.txt")
		check(err)
		str = string(dat)
	} else {
		str = testStr1
	}

	return strings.Split(str, "\n");
}
var prod = 1

func turn(monkey Monkey, monkeys []Monkey) {
	new_items := make([]int, 0)
	for _, item := range monkey.items {
		item = doOp(monkey, item) % prod

		var target int
		if (item % monkey.div == 0) {
			target = monkey.ifTrue
		} else {
			target = monkey.ifFalse
		}
		if target != monkey.num {
			monkeys[target].items = append(monkeys[target].items, item)
		} else {
			new_items = append(new_items, item)
		}
		monkeys[monkey.num].inspects++
	}
	monkeys[monkey.num].items = new_items
}

func round(monkeys []Monkey) {
	for _, monke := range monkeys {
		turn(monke, monkeys)
	}
}

const part2 = true

func main() {
	input := getInput(true)
	monkeys := make([]Monkey, 0, 6)
	curMonkeyStart := 0
	for i, s := range input {
		if s == "" {
			monkeys = append(monkeys, parseMonkey(input[curMonkeyStart:i]));
			curMonkeyStart = i + 1
		}
	}

	for _, m := range monkeys {
		prod *= m.div
	}

	for i := 0; i < 10000; i++ {
		round(monkeys)
	}
	mx := -1;
	sndmx := -2;
	for _, monke := range monkeys {
		if monke.inspects >= mx {
			sndmx = mx;
			mx = monke.inspects
		} else if monke.inspects >= sndmx {
			sndmx = monke.inspects
		}
	}

	fmt.Println(mx * sndmx)
	fmt.Println(monkeys)
	

}

/*

14399639990
18170818354
20709554856

*/