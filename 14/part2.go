package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strings"
)

const STEP_MAX = 2

func main() {
	file, err := os.Open("small_input.txt")
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

	polymerPairs := make(map[string]int)
	for i := 0; i < len(polymerStart)-1; i++ {
		polymerPairs[polymerStart[i:i+2]]++
	}

	DumpPairs(polymerPairs)

	for i := 1; i <= STEP_MAX; i++ {
		fmt.Println("START polymer expansion")
		DumpPairs(polymerPairs)
		polymerPairs = CreatePolymer(polymerPairs, pairRules)
		DumpPairs(polymerPairs)
		fmt.Println("END polymer expansion")
	}

	// charInPairTally := make(map[string]int)
	charTally := make(map[string]int)
	for pair, count := range polymerPairs {
		char0 := string(pair[0])
		char1 := string(pair[1])
		charTally[char0] += count
		charTally[char1] += count
		// charInPairTally[char0]++
		// charInPairTally[char1]++
	}

	fmt.Println("Char Tally")
	DumpPairs(charTally)
	// fmt.Println("Char In Pair Tally")
	// DumpPairs(charInPairTally)

	mm := make(map[string]int)
	for char, count := range charTally {
		res := float64(count) / 2.0
		mm[char] = int(math.Ceil(res))
	}

	fmt.Println("Correct Tally")
	DumpPairs(mm)

	// charTally := make(map[rune]int)
	// for _, c := range polymerStart {
	// 	charTally[c]++
	// }

	// counts := []int{}
	// for _, c := range charTally {
	// 	counts = append(counts, c)
	// }

	// sort.Ints(counts)

	// fmt.Println(counts)

	// min := counts[0]
	// max := counts[len(counts)-1]
	// fmt.Println("Result:", max-min)
}

func CreatePolymer(polymerPairs map[string]int, pairRules map[string]string) map[string]int {
	charCounts := make(map[string]int)
	newPolymerPairs := make(map[string]int)
	for pair := range polymerPairs {
		insert := pairRules[pair]
		pair1 := string(pair[0]) + insert
		pair2 := insert + string(pair[1])
		charCounts[string(pair[0])]++
		charCounts[string(pair[1])]++
		charCounts[insert]++

		fmt.Println("#######################")
		fmt.Printf("Origin Pair: %s | Pair 1: %s | Pair 2: %s\n", pair, pair1, pair2)

		if _, ok := polymerPairs[pair1]; ok {
			newPolymerPairs[pair1] += polymerPairs[pair1]
		} else {
			newPolymerPairs[pair1] += 1
		}

		if _, ok := polymerPairs[pair2]; ok {
			newPolymerPairs[pair2] += polymerPairs[pair2]
		} else {
			newPolymerPairs[pair2] += 1
		}

		// newPolymerPairs[pair1] += polymerPairs[pair1] + 1
		// newPolymerPairs[pair2] += polymerPairs[pair2] + 1
		DumpPairs(newPolymerPairs)
		fmt.Println("Current Char counts:")
		DumpPairs(charCounts)
	}

	fmt.Println("Final Char counts:")
	DumpPairs(charCounts)
	fmt.Println("#######################")

	return newPolymerPairs

	// This is too slow!
	// newPolymer := ""
	// for i := 0; i < len(polymerStart)-1; i++ {
	// 	pair := polymerStart[i : i+2]
	// 	insertElement := pairRules[pair]
	// 	newPolymer += string(pair[0]) + insertElement
	// }
	// newPolymer += string(polymerStart[len(polymerStart)-1])
	// return newPolymer
}

func DumpPairs(pairs map[string]int) {
	for k, v := range pairs {
		fmt.Printf("%s : %d\n", k, v)
	}
}
