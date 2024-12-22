package solution

import (
	"fmt"
	"strconv"
	"strings"
	"time"
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

func get(y string, registers map[string]int) int {
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
			registers[instruction.x] = get(instruction.y, registers)
		case "add":
			registers[instruction.x] += get(instruction.y, registers)
		case "mul":
			registers[instruction.x] *= get(instruction.y, registers)
		case "mod":
			registers[instruction.x] %= get(instruction.y, registers)
		case "rcv":
			if registers[instruction.x] > 0 {
				return fmt.Sprintf("%d", lastSound)
			}
		case "jgz":
			if get(instruction.x, registers) > 0 {
				pointer += get(instruction.y, registers) - 1
			}
		}
		pointer++
	}

	return fmt.Sprintf("%d", lastSound)
}

type Program struct {
	id        int
	registers map[string]int
	pc        int
	in        <-chan int
	out       chan<- int
	sent      int
}

func NewProgram(id int, output chan<- int, input <-chan int) *Program {
	return &Program{
		id: id,
		pc: 0,
		registers: map[string]int{
			"p": id,
		},
		in:   input,
		out:  output,
		sent: 0,
	}
}

var timeout = time.Millisecond * 200

func (p *Program) execute(instruction InstructionDay18) {
	switch instruction.kind {
	case "snd":
		p.sent++
		select {
		case p.out <- get(instruction.x, p.registers):
		case <-time.After(timeout):
			p.pc = -1
			return
		}
	case "set":
		p.registers[instruction.x] = get(instruction.y, p.registers)
	case "add":
		p.registers[instruction.x] += get(instruction.y, p.registers)
	case "mul":
		p.registers[instruction.x] *= get(instruction.y, p.registers)
	case "mod":
		p.registers[instruction.x] %= get(instruction.y, p.registers)
	case "rcv":
		select {
		case p.registers[instruction.x] = <-p.in:
		case <-time.After(timeout):
			p.pc = -1
			return
		}
	case "jgz":
		if get(instruction.x, p.registers) > 0 {
			p.pc += get(instruction.y, p.registers)
			return
		}
	}
	p.pc++
}

func (p *Program) RunUntilStop(instructions []InstructionDay18) int {
	for 0 <= p.pc && p.pc < len(instructions) {
		p.execute(instructions[p.pc])
	}
	return p.sent
}

func (d Day18) Part2(input string) string {
	instructions := d.parse(input)

	ch0 := make(chan int, 1_000_000)
	ch1 := make(chan int, 1_000_000)
	res := make(chan int, 1)

	p0 := NewProgram(0, ch0, ch1)
	p1 := NewProgram(1, ch1, ch0)

	go p0.RunUntilStop(instructions)
	go func() {
		res <- p1.RunUntilStop(instructions)
	}()

	return fmt.Sprintf("%d", <-res)
}
