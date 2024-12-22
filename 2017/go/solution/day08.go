package solution

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Day08 struct {
}

type Operation struct {
	kind string
}

var (
	Inc = Operation{"inc"}
	Dec = Operation{"dec"}
)

type Comparitor struct {
	kind string
}

var (
	LessThan           = Comparitor{"<"}
	GreaterThan        = Comparitor{">"}
	LessThanOrEqual    = Comparitor{"<="}
	GreaterThanOrEqual = Comparitor{">="}
	Equal              = Comparitor{"=="}
	NotEqual           = Comparitor{"!="}
)

type Condition struct {
	register   string
	comparitor Comparitor
	value      int
}

func (c Condition) eval(registers map[string]int) bool {
	switch c.comparitor {
	case LessThan:
		return registers[c.register] < c.value
	case GreaterThan:
		return registers[c.register] > c.value
	case LessThanOrEqual:
		return registers[c.register] <= c.value
	case GreaterThanOrEqual:
		return registers[c.register] >= c.value
	case Equal:
		return registers[c.register] == c.value
	case NotEqual:
		return registers[c.register] != c.value
	}
	panic("unreachable")
}

type Instruction struct {
	register  string
	operation Operation
	amount    int
	condition Condition
}

func newInstructionFromLine(line string) Instruction {
	tokens := strings.Split(line, " ")
	if len(tokens) != 7 {
		panic(fmt.Sprintf("invalid line: %s", line))
	}

	amount, err := strconv.Atoi(tokens[2])
	if err != nil {
		panic(err)
	}

	value, err := strconv.Atoi(tokens[6])
	if err != nil {
		panic(err)
	}

	return Instruction{
		register:  tokens[0],
		operation: Operation{tokens[1]},
		amount:    amount,
		condition: Condition{
			register:   tokens[4],
			comparitor: Comparitor{tokens[5]},
			value:      value,
		},
	}
}

func (d Day08) parse(input string) []Instruction {
	lines := strings.Split(input, "\n")
	instructions := make([]Instruction, len(lines))
	for i, line := range lines {
		instructions[i] = newInstructionFromLine(line)
	}
	return instructions
}

func (d Day08) process(input string) (int, int) {
	instructions := d.parse(input)
	registers := map[string]int{}

	maxAtAnyPoint := 0
	for _, instruction := range instructions {
		if instruction.condition.eval(registers) {
			switch instruction.operation {
			case Inc:
				registers[instruction.register] += instruction.amount
			case Dec:
				registers[instruction.register] -= instruction.amount
			}

			if registers[instruction.register] > maxAtAnyPoint {
				maxAtAnyPoint = registers[instruction.register]
			}
		}
	}

	// find max register value
	maxAtEnd := math.MinInt
	for _, v := range registers {
		if v > maxAtEnd {
			maxAtEnd = v
		}
	}

	return maxAtEnd, maxAtAnyPoint
}

func (d Day08) Part1(input string) string {
	max, _ := d.process(input)
	return fmt.Sprintf("%d", max)
}

func (d Day08) Part2(input string) string {
	_, max := d.process(input)
	return fmt.Sprintf("%d", max)
}
