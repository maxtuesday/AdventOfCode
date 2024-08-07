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

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func (d Day03) Part2(input string) string {

	return ""
}
