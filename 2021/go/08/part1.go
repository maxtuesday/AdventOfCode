package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

// Unique lengths:
// 1 -> 2
// 4 -> 4
// 7 -> 3
// 8 -> 7

// Other num lengths
// 0 -> 6
// 2 -> 5
// 3 -> 5
// 5 -> 5
// 6 -> 6
// 9 -> 6

func main() {
	file, err := os.Open("../../input/day08.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	patterns := [][]string{}
	for scanner.Scan() {
		line := scanner.Text()
		// get last nums
		numsStr := strings.Split(line, "| ")[1]
		nums := strings.Split(numsStr, " ")
		patterns = append(patterns, nums)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	count := 0
	for _, nums := range patterns {
		for _, num := range nums {
			l := len(num)
			if l == 2 || l == 4 || l == 3 || l == 7 {
				count++
			}
		}
	}

	fmt.Println("Number of 1, 4, 7, or 8s:", count)

}
