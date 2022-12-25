package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	input := readInput()
	solve(input)
}

func readInput() string {
	b, _ := os.ReadFile("../../input/10/input.txt")
	return string(b)
}

type CPU struct {
	Cycle          int
	X              int
	SignalStrength int
	Display        [][]string
}

func NewCPU() *CPU {
	display := make([][]string, 6)
	for i := range display {
		display[i] = make([]string, 40)
		for j := range display[i] {
			display[i][j] = "."
		}
	}
	return &CPU{
		X:       1,
		Display: display,
	}
}

func (cpu *CPU) Add(num int) {
	cpu.X += num
}

func (cpu *CPU) UpdateCycle() {
	cpu.Cycle++
	if 20 <= cpu.Cycle && (cpu.Cycle-20)%40 == 0 {
		cpu.SignalStrength += cpu.Cycle * cpu.X
	}
	cpu.UpdateDisplay()
}

func (cpu *CPU) UpdateDisplay() {
	factor := cpu.Cycle - 1
	pixelPos := factor % 40
	row := factor / 40
	spriteCenter := cpu.X
	if spriteCenter-1 <= pixelPos &&
		pixelPos <= spriteCenter+1 {
		cpu.Display[row][pixelPos] = "#"
	}
}

func (cpu *CPU) PrintDisplay() {
	for i := range cpu.Display {
		for j := range cpu.Display[i] {
			fmt.Printf("%s", cpu.Display[i][j])
		}
		fmt.Println()
	}
	fmt.Println()
}

func solve(input string) {
	cpu := NewCPU()
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		if line == "noop" {
			cpu.UpdateCycle()
		} else {
			cpu.UpdateCycle()
			cpu.UpdateCycle()
			n, _ := strconv.Atoi(strings.Split(line, " ")[1])
			cpu.Add(n)
		}
	}
	fmt.Printf("Part 1: %d\n", cpu.SignalStrength)
	fmt.Printf("Part 2:\n")
	cpu.PrintDisplay()
}
