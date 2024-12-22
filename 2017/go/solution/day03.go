package solution

import (
	"fmt"
	"math"
	"strconv"
)

type Day03 struct {
}

/*

Data is stored in a spiral pattern
Length of sides increase by 2 each "layer"
Ex: 1, 3, 5...

The last number for each side will be the square of the side length.
Ex:
1 -> 1
3 -> 9
5 -> 25

17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23---> ...

sqrt(12) ~= 3.46 -> ceil -> 4 -> if even, add 1 -> 5 (this is the side length)
How do we figure out distance from center vert/horizontal?

We could backtrack from the side lenght final location.
Side length is 5, so start with 25.
25 - 12 = 13
We need to go back 13 spaces.
The current coordinates for 25 are (2, -2)
Coordinates are generated from side length - 1 / 2.
((s-1) / 2, -(s-1)/2)

Start with going left, then up, then right, then down until we exhust the difference.
Change direction after we exhust the side length - 1.

*/

func (d Day03) Part1(input string) string {
	target, err := strconv.Atoi(input)
	if err != nil {
		panic(err)
	}

	sqrt := math.Sqrt(float64(target))
	S := int(math.Ceil(sqrt))
	if S%2 == 0 {
		S++
	}

	// backtrack
	depth := (S - 1) / 2
	coords := []int{depth, -depth}

	diff := S*S - target

	// x , y delta
	direction := 1 // left
	delta := []int{-1, 0}
	steps := 0
	for i := 0; i < diff; i++ {
		coords[0] += delta[0]
		coords[1] += delta[1]
		steps++
		if steps == S-1 {
			steps = 0
			switch direction {
			case 1: // left
				delta = []int{0, 1} // change to up
				direction = 2
			case 2: // up
				delta = []int{1, 0} // change to right
				direction = 3
			case 3: // right
				delta = []int{0, -1} // change to down
				direction = 4
			case 4: // down
				delta = []int{-1, 0} // change to left
				direction = 1
			default:
				panic("unreachable")
			}
		}
	}

	distance := abs(coords[0]) + abs(coords[1])
	return fmt.Sprintf("%d", distance)
}

func (d Day03) Part2(input string) string {
	// Simulate spiral?
	target, err := strconv.Atoi(input)
	if err != nil {
		panic(err)
	}

	mem := map[[2]int]int{}
	mem[[2]int{0, 0}] = 1
	mem[[2]int{1, 0}] = 1

	deltas := [][]int{
		{0, 1},  // up
		{-1, 0}, // left
		{0, -1}, // down
		{1, 0},  // right
	}

	// move in a spiral
	pos := [2]int{1, 0}
	S := 3 // increase this by 2

	// opertaions:
	// up by S-2
	// left by S-1
	// down by S-1
	// right by S

	for {
		// up
		for j := 0; j < S-2; j++ {
			pos[0] += deltas[0][0]
			pos[1] += deltas[0][1]

			// apply value
			mem[pos] = sumAdj(pos, mem)

			if mem[pos] > target {
				return fmt.Sprintf("%d", mem[pos])
			}
		}
		// left
		for j := 0; j < S-1; j++ {
			pos[0] += deltas[1][0]
			pos[1] += deltas[1][1]

			// apply value
			mem[pos] = sumAdj(pos, mem)

			if mem[pos] > target {
				return fmt.Sprintf("%d", mem[pos])
			}
		}
		// down
		for j := 0; j < S-1; j++ {
			pos[0] += deltas[2][0]
			pos[1] += deltas[2][1]

			// apply value
			mem[pos] = sumAdj(pos, mem)

			if mem[pos] > target {
				return fmt.Sprintf("%d", mem[pos])
			}
		}
		// right
		for j := 0; j < S; j++ {
			pos[0] += deltas[3][0]
			pos[1] += deltas[3][1]

			// apply value
			mem[pos] = sumAdj(pos, mem)

			if mem[pos] > target {
				return fmt.Sprintf("%d", mem[pos])
			}
		}

		// at end of finishing layer of spiral
		S += 2
	}
}

func sumAdj(pos [2]int, mem map[[2]int]int) int {
	sum := 0
	for i := -1; i <= 1; i++ {
		for j := -1; j <= 1; j++ {
			sum += mem[[2]int{pos[0] + i, pos[1] + j}]
		}
	}
	return sum
}
