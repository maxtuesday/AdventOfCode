package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("../../input/day09.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	heightMap := [][]int{}
	for scanner.Scan() {
		lineNums := []int{}
		for _, s := range strings.Split(scanner.Text(), "") {
			n, _ := strconv.Atoi(s)
			lineNums = append(lineNums, n)
		}
		heightMap = append(heightMap, lineNums)
	}

	sum := 0
	for row := range heightMap {
		for col := range heightMap[row] {
			var left, right, up, down int
			level := heightMap[row][col]

			if row == 0 {
				up = level + 1
			} else {
				up = heightMap[row-1][col]
			}

			if row+1 == len(heightMap) {
				down = level + 1
			} else {
				down = heightMap[row+1][col]
			}

			if col == 0 {
				left = level + 1
			} else {
				left = heightMap[row][col-1]
			}

			if col+1 == len(heightMap[row]) {
				right = level + 1
			} else {
				right = heightMap[row][col+1]
			}

			if level < left &&
				level < right &&
				level < up &&
				level < down {
				sum += level + 1
			}
		}
	}

	fmt.Println("Result:", sum)
}
