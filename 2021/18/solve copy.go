package main

import (
	"fmt"
	"os"
)

const inputFile = "sample_input.txt"

type Snailfish struct {
	depths map[int]int
	nums   []Num
}

func (s *Snailfish) GetFirstGreaterThan10() *Num {
	for i := range s.nums {
		if s.nums[i].val >= 10 {
			return &s.nums[i]
		}
	}
	return nil
}

type Num struct {
	val   int
	depth int
}

func LoadData(filename string) {

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

func Parse(input string) *Snailfish {
	depth := 0
	depths := make(map[int]int)
	nums := []Num{}
	igte10 := []int{}
	for i, c := range input {
		switch c {
		case '[':
			depth++
		case ']':
			depth--
		case ',':
		default: // Number
			n := int(c - '0')
			if n >= 10 {
				igte10 = append(igte10, i)
			}
			nums = append(nums, Num{val: n, depth: depth})
			depths[depth]++
		}
	}
	return &Snailfish{depths: depths, nums: nums, indexGte10: igte10}
}

func Reduce(snailfish *Snailfish) {
	fmt.Println("Reduce")
	fmt.Printf("!!! Snailfish: %+v\n", snailfish)
	if snailfish.depths[5] > 0 {
		Explode(snailfish)
	} else if snailfish.GetFirstGreaterThan10() != nil {
		Split(snailfish)
	} else {
		fmt.Printf("Reduced, no other actions: %+v\n", snailfish)
		return
	}
	// time.Sleep(time.Second * 2)
	Reduce(snailfish)
}

// func Addition(a, b *Snailfish) *Snailfish {

// }

func Explode(snailfish *Snailfish) {
	fmt.Println("Explode")

	var a, b Num
	var a_before, b_after *Num
	var numsBefore, numsAfter, reduced []Num

	for i := range snailfish.nums {
		depth := snailfish.nums[i].depth
		if depth >= 5 {
			a = snailfish.nums[i]
			b = snailfish.nums[i+1]

			if i-1 >= 0 {
				a_before = &snailfish.nums[i-1]
				numsBefore = snailfish.nums[:i-1]
			}

			if i+3 <= len(snailfish.nums) {
				b_after = &snailfish.nums[i+2]
				numsAfter = snailfish.nums[i+3:]
			}

			// fmt.Printf("nums: %+v\n", snailfish.nums)
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

			// fmt.Printf("Nums: %+v\n", reduced)
			// fmt.Println()

			snailfish.nums = reduced
			snailfish.depths[depth] -= 2
			snailfish.depths[depth-1] += 2
			// fmt.Printf("!!! Snailfish: %+v\n", snailfish)
			return
		}
	}
}

func Split(snailfish *Snailfish) {
	fmt.Println("Split")
	// TODO: improve performance by saving the max num index in Snailfish struct
	var before, after []Num
	var reduced []Num
	for i := range snailfish.nums {
		val := snailfish.nums[i].val
		depth := snailfish.nums[i].depth
		if val == snailfish.GetMax() {
			// if i > 0 {
			before = snailfish.nums[:i]
			// }
			// if i <= len(snailfish.nums) {
			after = snailfish.nums[i+1:]
			// }

			fmt.Printf("num: %+v\n", snailfish.nums[i])
			fmt.Printf("nums before: %+v\n", before)
			fmt.Printf("nums after: %+v\n", after)

			a := Num{val: val / 2, depth: depth + 1}
			b := Num{val: val/2 + val%2, depth: depth + 1}

			reduced = append(reduced, before...)
			reduced = append(reduced, a)
			reduced = append(reduced, b)
			reduced = append(reduced, after...)

			fmt.Printf("%+v\n", reduced)
			os.Exit(0)
		}
	}
}

func Part1() {
	// input := "[[[[[9,8],1],2],3],4]"
	// input := "[7,[6,[5,[4,[3,2]]]]]"
	// input := "[[6,[5,[4,[3,2]]]],1]"
	// input := "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
	// input := "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
	input := "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"
	snailfish := Parse(input)
	Reduce(snailfish)
}
