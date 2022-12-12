package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	input := readInput()
	fmt.Printf("Part 1: %s\n", part1(input))
	fmt.Printf("Part 2: %s\n", part2(input))
}

func readInput() string {
	// b, _ := os.ReadFile("../../inputs/10/large_input.txt")
	b, _ := os.ReadFile("../../inputs/10/input.txt")
	return string(b)
}

type CPU struct {
	Cycle          int
	X              int
	SignalStrength int
	Display        [][]string
}

func (cpu *CPU) UpdateCycle() {
	cpu.Cycle++
	if 20 <= cpu.Cycle && (cpu.Cycle-20)%40 == 0 {
		cpu.SignalStrength += cpu.Cycle * cpu.X
	}
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

func part1(input string) string {
	cpu := &CPU{X: 1}
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		if line == "noop" {
			cpu.UpdateCycle()
		} else {
			cpu.UpdateCycle()
			cpu.UpdateCycle()
			n, _ := strconv.Atoi(strings.Split(line, " ")[1])
			cpu.X += n
		}
	}
	return fmt.Sprintf("%d", cpu.SignalStrength)
}

func part2(input string) string {
	display := make([][]string, 6)
	for i := range display {
		display[i] = make([]string, 40)
		for j := range display[i] {
			display[i][j] = "."
		}
	}
	cpu := &CPU{
		X:       1,
		Display: display,
	}

	lines := strings.Split(input, "\n")
	for _, line := range lines {
		if line == "noop" {
			cpu.UpdateCycle()
			cpu.UpdateDisplay()
		} else {
			cpu.UpdateCycle()
			cpu.UpdateDisplay()
			cpu.UpdateCycle()
			cpu.UpdateDisplay()
			n, _ := strconv.Atoi(strings.Split(line, " ")[1])
			cpu.X += n
		}
	}

	cpu.PrintDisplay()
	return "^"
}
