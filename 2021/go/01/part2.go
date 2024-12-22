package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	b, err := os.ReadFile("../../input/day01.txt")
	if err != nil {
		log.Fatal(err)
	}

	data := string(b)
	dataSlice := strings.Split(data, "\n")

	count := 0
	levels := make([]int, len(dataSlice)-1)

	// convert levels to ints
	for i, str := range dataSlice {
		c, err := strconv.Atoi(str)
		if err != nil {
			fmt.Println(err)
			break
		}
		levels[i] = c
	}

	prevSum := levels[0] + levels[1] + levels[2]
	for i := range levels {
		if i+2 >= len(levels) {
			fmt.Println("no more 3 pair levels")
			break
		}
		sum := levels[i] + levels[i+1] + levels[i+2]
		if sum > prevSum {
			count++
		}
		prevSum = sum
	}

	fmt.Println(count)
}
