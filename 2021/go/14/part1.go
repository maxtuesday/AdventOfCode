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
	file, err := os.Open("../../input/day14.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	// Read polymer
	scanner.Scan()
	polymerStr := scanner.Text()
	// Scan next empty line
	scanner.Scan()

	pairRules := make(map[string]string)
	for scanner.Scan() {
		split := strings.Split(scanner.Text(), " -> ")
		pairRules[split[0]] = split[1]
	}

	polymerPairCounts := make(map[string]int)
	for i := 0; i < len(polymerStr)-1; i++ {
		polymerPairCounts[polymerStr[i:i+2]]++
	}
	elementCounts := make(map[string]int)
	for _, r := range polymerStr {
		elementCounts[string(r)]++
	}

	for i := 1; i <= STEP_MAX; i++ {
		polymerPairCounts = CreatePolymer(polymerPairCounts, pairRules, elementCounts)
	}

	counts := []int{}
	for _, c := range elementCounts {
		counts = append(counts, c)
	}

	sort.Ints(counts)

	min := counts[0]
	max := counts[len(counts)-1]
	fmt.Println("Result:", max-min)
}

func CreatePolymer(polymerPairCounts map[string]int, pairRules map[string]string, elementCounts map[string]int) map[string]int {
	polymerPairCountsNew := make(map[string]int)
	for pair := range polymerPairCounts {
		numPairs := polymerPairCounts[pair]
		insert := pairRules[pair]
		pair1 := string(pair[0]) + insert
		pair2 := insert + string(pair[1])
		polymerPairCountsNew[pair1] += numPairs
		polymerPairCountsNew[pair2] += numPairs
		elementCounts[insert] += numPairs
	}
	return polymerPairCountsNew
}

func DumpPairs(pairs map[string]int) {
	for k, v := range pairs {
		fmt.Printf("%s : %d\n", k, v)
	}
}
