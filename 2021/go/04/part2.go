package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Board struct {
	data    []string
	dataPos map[string]int
	marked  [25]string
}

func NewBoard(data []string) *Board {
	// gen dataPos map
	dataPos := make(map[string]int)
	for i, d := range data {
		dataPos[d] = i
	}
	return &Board{
		data:    data,
		dataPos: dataPos,
	}
}

func (b *Board) Mark(s string) {
	if pos, ok := b.dataPos[s]; ok {
		b.marked[pos] = s
	}
}

func ContainsEmpty(slice []string) bool {
	for _, s := range slice {
		if s == "" {
			return true
		}
	}
	return false
}

func (b *Board) Won() bool {
	// Check Rows
	if !ContainsEmpty(b.marked[0:5]) ||
		!ContainsEmpty(b.marked[5:10]) ||
		!ContainsEmpty(b.marked[10:15]) ||
		!ContainsEmpty(b.marked[15:20]) ||
		!ContainsEmpty(b.marked[20:25]) {
		return true
	}

	for i := 0; i < 5; i++ {
		if b.marked[i] != "" &&
			b.marked[i+5] != "" &&
			b.marked[i+10] != "" &&
			b.marked[i+15] != "" &&
			b.marked[i+20] != "" {
			return true
		}
	}
	return false
}

func (b *Board) Dump() {
	indices := []int{}
	for i, e := range b.marked {
		if e != "" {
			indices = append(indices, i)
		}
	}

	for i, e := range b.data {
		flag := false
		for _, in := range indices {
			if i == in {
				flag = true
				break
			}
		}
		if flag {
			fmt.Printf("[ %s ]\t", e)
		} else {
			fmt.Printf("  %s  \t", e)
		}

		if (i+1)%5 == 0 {
			fmt.Println()
		}
	}
	fmt.Println()
}

func (b *Board) GetUnmarked() []string {
	// Get indices for numbers that are not seen
	indices := []int{}
	for i, e := range b.marked {
		if e == "" {
			indices = append(indices, i)
		}
	}
	nums := []string{}
	for _, index := range indices {
		nums = append(nums, b.data[index])
	}
	return nums
}

func main() {
	file, err := os.Open("../../input/day04.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	nums := strings.Split(scanner.Text(), ",")

	lines := []string{}
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) != 0 {
			lines = append(lines, line)
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	// create boards
	boards := []*Board{}
	for i := 0; i < len(lines); i += 5 {
		rows := lines[i : i+5]
		boardStr := []string{}
		for _, row := range rows {
			boardStr = append(boardStr, strings.Split(row, " ")...)
		}

		data := []string{}
		for _, b := range boardStr {
			if len(b) != 0 {
				data = append(data, b)
			}
		}

		boards = append(boards, NewBoard(data))
	}

	fmt.Println(len(boards))

	b, winningNumStr := play(nums, boards)
	if b == nil {
		log.Fatal("No winning board!")
	}

	fmt.Println("Winning board:")
	b.Dump()

	fmt.Println("Winning Num:", winningNumStr)

	fmt.Println("Nums not visited:")
	sum := 0
	for _, e := range b.GetUnmarked() {
		n, _ := strconv.Atoi(e)
		sum += n
	}
	fmt.Println(sum)

	winningNum, err := strconv.Atoi(winningNumStr)

	fmt.Println("Result:", sum*winningNum)
}

func play(nums []string, boards []*Board) (*Board, string) {
	winningBoards := []int{}
	winningNums := []string{}
	for _, num := range nums {
		fmt.Println("Draw:", num)
		for i, board := range boards {
			skip := false
			for _, wb := range winningBoards {
				if wb == i {
					skip = true
				}
			}
			if !skip {
				board.Mark(num)
				if board.Won() {
					winningBoards = append(winningBoards, i)
					winningNums = append(winningNums, num)
				}
			}
		}
	}
	return boards[winningBoards[len(winningBoards)-1]], winningNums[len(winningNums)-1]
}

func prettyPrint(slice interface{}) {
	fmt.Printf("%#v\n", slice)
}
