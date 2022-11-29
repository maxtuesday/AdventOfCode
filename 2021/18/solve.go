package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

const inputFile = "sample_input.txt"

type Num struct {
	val   int
	depth int
}

func LoadData(filename string) []string {
	file, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	lines := []string{}
	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)
	}
	return lines
}

func main() {
	Part1()
}

func Max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func Parse(input string) []Num {
	depth := 0
	nums := []Num{}
	for _, c := range input {
		switch c {
		case '[':
			depth++
		case ']':
			depth--
		case ',':
		default: // Number
			n := int(c - '0')
			nums = append(nums, Num{val: n, depth: depth})
		}
	}
	return nums
}

func Reduce(nums []Num) []Num {
	// fmt.Printf("%+v\n", nums)
	explode_nums, reduced := Explode(nums)
	if !reduced {
		split_nums, split := Split(explode_nums)
		if !split {
			// fmt.Printf("Reduced, no other actions: %+v\n", split_nums)
			return split_nums
		}
		// time.Sleep(time.Second * 1)
		return Reduce(split_nums)
	} else {
		// time.Sleep(time.Second * 1)
		return Reduce(explode_nums)
	}
}

func Addition(a, b []Num) []Num {
	n := append(a, b...)
	for i := range n {
		n[i].depth++
	}
	return n
}

func Explode(nums []Num) ([]Num, bool) {
	// fmt.Println("Explode")

	var a, b Num
	var a_before, b_after *Num
	var numsBefore, numsAfter, reduced []Num

	for i := range nums {
		depth := nums[i].depth
		if depth >= 5 {
			a = nums[i]
			b = nums[i+1]

			if i-1 >= 0 {
				a_before = &nums[i-1]
				numsBefore = nums[:i-1]
			}

			if i+3 <= len(nums) {
				b_after = &nums[i+2]
				numsAfter = nums[i+3:]
			}

			// fmt.Printf("nums: %+v\n", nums)
			// fmt.Printf(" a: %+v\n", a)
			// fmt.Printf(" b: %+v\n", b)
			// fmt.Printf(" a_before: %+v\n", a_before)
			// fmt.Printf(" b_after: %+v\n", b_after)
			// fmt.Printf(" numsBefore: %+v\n", numsBefore)
			// fmt.Printf(" numsAfter: %+v\n", numsAfter)
			// fmt.Println()

			if a_before == nil {
				a.val = 0
				a_before = &Num{val: 0, depth: b_after.depth}
			}
			if b_after == nil {
				b.val = 0
				b_after = &Num{val: 0, depth: a_before.depth}
			}

			a_before.val += a.val
			b_after.val += b.val

			reduced = append(reduced, numsBefore...)
			reduced = append(reduced, *a_before)
			if a.depth-1 != a_before.depth {
				reduced = append(reduced, Num{val: 0, depth: a.depth - 1})
			}
			if b.depth-1 != b_after.depth {
				reduced = append(reduced, Num{val: 0, depth: b.depth - 1})
			}
			reduced = append(reduced, *b_after)
			reduced = append(reduced, numsAfter...)

			fmt.Printf("After Explode: %+v\n", reduced)
			return reduced, true
		}
	}
	return nums, false
}

func Split(nums []Num) ([]Num, bool) {
	// fmt.Println("Split")
	// TODO: improve performance by saving the max num index in Snailfish struct
	var before, after []Num
	var reduced []Num
	for i := range nums {
		val := nums[i].val
		depth := nums[i].depth
		if val >= 10 {
			// if i > 0 {
			before = nums[:i]
			// }
			// if i <= len(nums) {
			after = nums[i+1:]
			// }

			// fmt.Printf("num: %+v\n", nums[i])
			// fmt.Printf("nums before: %+v\n", before)
			// fmt.Printf("nums after: %+v\n", after)

			a := Num{val: val / 2, depth: depth + 1}
			b := Num{val: val/2 + val%2, depth: depth + 1}

			reduced = append(reduced, before...)
			reduced = append(reduced, a)
			reduced = append(reduced, b)
			reduced = append(reduced, after...)

			fmt.Printf("After Split: %+v\n", reduced)
			return reduced, true
		}
	}
	return nums, false
}

func Part1() {
	// input := "[[[[[9,8],1],2],3],4]"
	// input := "[7,[6,[5,[4,[3,2]]]]]"
	// input := "[[6,[5,[4,[3,2]]]],1]"
	// input := "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
	// input := "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
	// input := "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"

	// a_input := "[[[[4,3],4],4],[7,[[8,4],9]]]"
	// b_input := "[1,1]"
	// a := Parse(a_input)
	// b := Parse(b_input)
	// add := Addition(a, b)
	// fmt.Printf("%+v\n", add)
	// snailfish := Parse(input)
	// r := Reduce(snailfish)
	// fmt.Printf("r: %+v\n", r)

	// Parse list of snailfish lines
	snailfish := LoadData(inputFile)
	a := Parse(snailfish[0])
	b := Parse(snailfish[1])
	ab := Addition(a, b)
	sum := Reduce(ab)
	for i := 2; i < len(snailfish); i++ {
		sum = Reduce(Addition(sum, Parse(snailfish[i])))
	}
	fmt.Printf("Part 1: %+v\n", sum)
}
