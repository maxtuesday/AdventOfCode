package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strings"
)

const STEP_MAX = 10

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	// Read polymer
	scanner.Scan()
	polymerStart := scanner.Text()
	// Scan next empty line
	scanner.Scan()

	pairRules := make(map[string]string)
	for scanner.Scan() {
		split := strings.Split(scanner.Text(), " -> ")
		pairRules[split[0]] = split[1]
	}

	for i := 1; i <= STEP_MAX; i++ {
		polymerStart = CreatePolymer(polymerStart, pairRules)
	}

	charTally := make(map[rune]int)
	for _, c := range polymerStart {
		charTally[c]++
	}

	counts := []int{}
	for _, c := range charTally {
		counts = append(counts, c)
	}

	sort.Ints(counts)

	min := counts[0]
	max := counts[len(counts)-1]
	fmt.Println("Result:", max-min)
}

func CreatePolymer(polymerStart string, pairRules map[string]string) string {
	newPolymer := ""
	for i := 0; i < len(polymerStart)-1; i++ {
		pair := polymerStart[i : i+2]
		insertElement := pairRules[pair]
		newPolymer += string(pair[0]) + insertElement
	}
	newPolymer += string(polymerStart[len(polymerStart)-1])
	return newPolymer
}
