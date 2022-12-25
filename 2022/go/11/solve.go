package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	input := readInput()
	fmt.Printf("Part 1: %s\n", part1(input))
	fmt.Printf("Part 2: %s\n", part2(input))
}

func readInput() string {
	b, _ := os.ReadFile("../../input/11/input.txt")
	return string(b)
}

type Operation struct {
	Op     string
	Factor string
}

type Monkey struct {
	Items        []int
	Operation    Operation
	TestFactor   int
	Destinations []int
}

func (m Monkey) String() string {
	return fmt.Sprintf("Items: %v\n Operation: %+v\n TestFactor: %v\n Destinations: %v\n",
		m.Items, m.Operation, m.TestFactor, m.Destinations)
}

// TODO: Refactor to be less nasty
// Use Regex?
func newMonkey(s string) *Monkey {
	parts := strings.Split(s, "\n")[1:] // skip 'Monkey X:'
	startingItemsStr := strings.Split(parts[0], ":")[1]
	str := strings.ReplaceAll(startingItemsStr, ", ", " ")
	fields := strings.Fields(str)
	items := []int{}
	for _, item := range fields {
		n, _ := strconv.Atoi(item)
		items = append(items, n)
	}
	opertaionStr := strings.Split(parts[1], "= old ")[1]
	op := Operation{
		Op:     string(opertaionStr[0]),
		Factor: strings.Trim(opertaionStr[1:], " "),
	}
	testStr := strings.Split(parts[2], "divisible by ")[1]
	testFactor, _ := strconv.Atoi(testStr)
	testTrueStr := strings.Split(parts[3], "monkey ")[1]
	testTrue, _ := strconv.Atoi(testTrueStr)
	testFalseStr := strings.Split(parts[4], "monkey ")[1]
	testFalse, _ := strconv.Atoi(testFalseStr)
	destinations := []int{
		testTrue,
		testFalse,
	}
	return &Monkey{
		Items:        items,
		Operation:    op,
		TestFactor:   testFactor,
		Destinations: destinations,
	}
}

func parseMonkeys(input string) []*Monkey {
	monkeys := []*Monkey{}
	monkeysRaw := strings.Split(input, "\n\n")
	for _, monkeyRaw := range monkeysRaw {
		monkeys = append(monkeys, newMonkey(monkeyRaw))
	}
	return monkeys
}

func printMonkeys(monkeys []*Monkey) {
	for _, monkey := range monkeys {
		fmt.Println(monkey)
	}
}

func applyOperation(monkey *Monkey, item int) int {
	operation := monkey.Operation
	factor := item
	if operation.Factor != "old" {
		factor, _ = strconv.Atoi(operation.Factor)
	}
	switch operation.Op {
	case "+":
		return item + factor
	case "-":
		return item - factor
	case "*":
		return item * factor
	case "/":
		return item / factor
	default:
		panic(fmt.Sprintf("Not supported operation: %s\n", operation.Op))
	}
}

func GCD(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func LCM_Monkeys(monkeys []*Monkey) int {
	res := 1
	for _, monkey := range monkeys {
		res = (res * monkey.TestFactor) / GCD(res, monkey.TestFactor)
	}
	return res
}

func monkeyRound(monkeys []*Monkey, worryFactor int, counts []int, lcm int) {
	// Loop over monkeys, per monkey:
	//   1. Pop item from monkey items
	//   2. apply operation to item value
	//   3. divide by 3
	//   4. Check test
	//   5. append item (with new value) to next monkey based on test result
	for i, monkey := range monkeys {
		counts[i] += len(monkey.Items)
		for _, item := range monkey.Items {
			newItemValue := (applyOperation(monkey, item) / worryFactor) % lcm
			var destMonkey *Monkey
			if newItemValue%monkey.TestFactor == 0 {
				destMonkey = monkeys[monkey.Destinations[0]]
			} else {
				destMonkey = monkeys[monkey.Destinations[1]]
			}
			destMonkey.Items = append(destMonkey.Items, newItemValue)
		}
		monkey.Items = []int{}
	}
}

func part1(input string) string {
	monkeys := parseMonkeys(input)
	lcm := LCM_Monkeys(monkeys)
	counts := make([]int, len(monkeys))
	for i := 0; i < 20; i++ {
		monkeyRound(monkeys, 3, counts, lcm)
	}

	sort.Ints(counts)

	l := len(counts)
	ans := counts[l-1] * counts[l-2]

	return fmt.Sprintf("%d", ans)
}

func part2(input string) string {
	monkeys := parseMonkeys(input)
	lcm := LCM_Monkeys(monkeys)
	counts := make([]int, len(monkeys))
	for i := 1; i <= 10000; i++ {
		monkeyRound(monkeys, 1, counts, lcm)
	}

	sort.Ints(counts)

	l := len(counts)
	ans := counts[l-1] * counts[l-2]

	return fmt.Sprintf("%d", ans)
}
