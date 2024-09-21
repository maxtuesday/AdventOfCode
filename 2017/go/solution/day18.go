package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day18 struct {
}

type InstructionDay18 struct {
	kind string
	x    string
	y    string
}

func (d Day18) parse(input string) []InstructionDay18 {
	instructions := make([]InstructionDay18, 0)
	for _, line := range strings.Split(input, "\n") {
		tokens := strings.Fields(line)
		var y string
		if len(tokens) == 3 {
			y = tokens[2]
		}
		instructions = append(instructions, InstructionDay18{
			kind: tokens[0],
			x:    tokens[1],
			y:    y,
		})
	}
	return instructions
}

func (d Day18) tryParseY(y string, registers map[string]int) int {
	val, err := strconv.Atoi(y)
	if err != nil {
		// this is a register
		val = registers[y]
	}
	return val
}

func (d Day18) Part1(input string) string {
	instructions := d.parse(input)
	registers := map[string]int{}
	lastSound := 0
	pointer := 0

	for 0 <= pointer && pointer < len(instructions) {
		instruction := instructions[pointer]
		switch instruction.kind {
		case "snd":
			lastSound = registers[instruction.x]
		case "set":
			registers[instruction.x] = d.tryParseY(instruction.y, registers)
		case "add":
			registers[instruction.x] += d.tryParseY(instruction.y, registers)
		case "mul":
			registers[instruction.x] *= d.tryParseY(instruction.y, registers)
		case "mod":
			registers[instruction.x] %= d.tryParseY(instruction.y, registers)
		case "rcv":
			if registers[instruction.x] > 0 {
				return fmt.Sprintf("%d", lastSound)
			}
		case "jgz":
			if registers[instruction.x] > 0 {
				pointer += d.tryParseY(instruction.y, registers) - 1
			}
		}
		pointer++
	}

	return fmt.Sprintf("%d", lastSound)
}

func (d Day18) Part2(input string) string {
	return ""
}
