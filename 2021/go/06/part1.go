package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

const DAYS = 80
const LIFE_LEN = 9

func main() {
	file, err := os.Open("../../input/day06.txt")
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

	currGen := [LIFE_LEN]int{}
	fishCyclesStr := strings.Split(input, ",")
	for _, fcs := range fishCyclesStr {
		n, _ := strconv.Atoi(fcs)
		currGen[n]++
	}

	for d := 0; d < DAYS; d++ {
		nextGen := [LIFE_LEN]int{}
		for f := 0; f < LIFE_LEN; f++ {
			if f == 0 {
				nextGen[8] += currGen[f]
				nextGen[6] += currGen[f]
			} else {
				nextGen[f-1] += currGen[f]
			}
		}
		currGen = nextGen
	}

	totalFish := 0
	for i := 0; i < LIFE_LEN; i++ {
		totalFish += currGen[i]
	}

	fmt.Printf("Day: %d | Total Fish: %d\n", DAYS, totalFish)

}
