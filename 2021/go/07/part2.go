package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("../../input/day07.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	input := scanner.Text()

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	// Parse input
	crabs := make(map[int]int)
	iSplit := strings.Split(input, ",")
	minPos := math.MaxInt
	maxPos := -1
	for _, i := range iSplit {
		n, _ := strconv.Atoi(i)
		crabs[n]++
		if minPos > n {
			minPos = n
		}
		if maxPos < n {
			maxPos = n
		}
	}

	// brute force
	minFuel := math.MaxInt
	for pos := minPos; pos <= maxPos; pos++ {
		fuel := 0
		for p, c := range crabs {
			// calc fuel per crab
			dist := int(math.Abs(float64((pos - p))))
			cFuel := (dist * (dist + 1)) / 2
			fuel += int(cFuel * c)
		}
		minFuel = int(math.Min(float64(minFuel), float64(fuel)))
	}

	fmt.Println("Min Fuel:", minFuel)

}
