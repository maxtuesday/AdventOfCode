package solution

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Day02 struct {
}

func (d Day02) Part1(input string) string {
	sum := 0
	for _, line := range strings.Split(input, "\n") {
		min := math.MaxInt
		max := math.MinInt
		for _, num := range strings.Fields(line) {
			n, err := strconv.Atoi(num)
			if err != nil {
				panic(err)
			}

			if n < min {
				min = n
			}
			if n > max {
				max = n
			}
		}
		sum += max - min
	}
	return fmt.Sprintf("%d", sum)
}

func (d Day02) Part2(input string) string {
	sum := 0
	for _, line := range strings.Split(input, "\n") {
		numsRaw := strings.Fields(line)
		nums := make([]int, len(numsRaw))
		for i := range numsRaw {
			n, err := strconv.Atoi(numsRaw[i])
			if err != nil {
				panic(err)
			}
			nums[i] = n
		}
		sum += getDivision(nums)
	}
	return fmt.Sprintf("%d", sum)
}

func getDivision(nums []int) int {
	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			// check if numbers evenly divide each other
			a, b := nums[i], nums[j]
			if a < b {
				// swap so a is larger
				a, b = b, a
			}
			if a%b == 0 {
				return a / b
			}
		}
	}
	panic("unreachable")
}
