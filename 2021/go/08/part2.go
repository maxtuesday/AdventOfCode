package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("../../input/day08.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	entries := [][]string{}
	for scanner.Scan() {
		line := scanner.Text()
		// get last nums
		entrySplit := strings.Split(line, " | ")
		entries = append(entries, []string{entrySplit[0], entrySplit[1]})
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	/*

		Possible Numbers based on known patterns
		1 (l 2): 0, 3, 4, 7, 8, 9
		-> 4,7,8 all include these segments, whatever those are, we can figure out the other segment locations

		4 (l 4): 8, 9
		-> We can find 9. Find segment that has length 6 and contains segments from 4, that is 9.

		7 (l 3): 0, 3, 9
		-> We can find 3. Find segment that has length 5 and conatins segments from 7, that is 3.

		8 (l 7): 8 Is unique

		Nums still unknown:
		0, 2, 5, 6

		0, 6 (l 6)
		2, 5 (l 5)

		Find 5 by using 9.
		5 contains all segments from 9 besided one segment

	*/

	sum := 0

	for _, e := range entries {
		// entry contains two elements
		// [0] = all unique segments
		// [1] = four numbers we need for result sum
		segments := strings.Split(e[0], " ")
		digitsKnown := [10]string{}
		digitsUnknown := []string{}

		for _, seg := range segments {
			// Take unique segments and see if we can figure out what number they are supposed to be
			l := len(seg)
			s := sortChars(seg)
			switch l {
			case 2:
				digitsKnown[1] = s
			case 4:
				digitsKnown[4] = s
			case 3:
				digitsKnown[7] = s
			case 7:
				digitsKnown[8] = s
			default:
				digitsUnknown = append(digitsUnknown, s)
			}
		}

		// Find 3 and 9
		for i, d := range digitsUnknown {
			if len(d) == 5 && containsAll(d, digitsKnown[7]) {
				digitsKnown[3] = d
				digitsUnknown[i] = ""
			} else if len(d) == 6 && containsAll(d, digitsKnown[4]) {
				digitsKnown[9] = d
				digitsUnknown[i] = ""
			}
		}

		// Find 2 and 5
		for i, d := range digitsUnknown {
			if len(d) == 5 {
				if containsAll(digitsKnown[9], d) {
					digitsKnown[5] = d
					digitsUnknown[i] = ""
				} else {
					digitsKnown[2] = d
					digitsUnknown[i] = ""
				}
			}
		}

		// Find 0 and 6
		for i, d := range digitsUnknown {
			if len(d) == 6 {
				if containsAll(d, digitsKnown[7]) {
					digitsKnown[0] = d
					digitsUnknown[i] = ""
				} else {
					digitsKnown[6] = d
					digitsUnknown[i] = ""
				}
			}
		}

		digitMap := make(map[string]int)
		for i := range digitsKnown {
			digitMap[digitsKnown[i]] = i
		}

		targetNums := strings.Split(e[1], " ")
		num := ""
		for i := range targetNums {
			d := sortChars(targetNums[i])
			num += fmt.Sprintf("%d", digitMap[d])
		}
		n, _ := strconv.Atoi(num)
		sum += n
	}

	fmt.Println("Result:", sum)
}

func sortChars(w string) string {
	s := []rune(w)
	sort.Slice(s, func(i, j int) bool { return s[i] < s[j] })
	return string(s)
}

func containsAll(base string, needles string) bool {
	containsAllChars := true
	for _, c := range []rune(needles) {
		if !strings.ContainsRune(base, c) {
			containsAllChars = false
		}
	}
	return containsAllChars
}
