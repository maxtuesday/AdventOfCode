package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Segment struct {
	x1, x2, y1, y2 int
}

func NewSegment(p1 []string, p2 []string) *Segment {
	x1, _ := strconv.Atoi(p1[0])
	y1, _ := strconv.Atoi(p1[1])
	x2, _ := strconv.Atoi(p2[0])
	y2, _ := strconv.Atoi(p2[1])
	return &Segment{
		x1: x1,
		y1: y1,
		x2: x2,
		y2: y2,
	}
}

func (s *Segment) Dump() {
	fmt.Printf("(%d, %d) -> (%d, %d)\n", s.x1, s.y1, s.x2, s.y2)
}

func main() {
	file, err := os.Open("../../input/day05.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	segments := []*Segment{}

	for scanner.Scan() {
		// Parse segment
		line := scanner.Text()
		noSpaces := strings.ReplaceAll(line, " ", "")
		pairs := strings.Split(noSpaces, "->")
		p1 := strings.Split(pairs[0], ",")
		p2 := strings.Split(pairs[1], ",")
		segments = append(segments, NewSegment(p1, p2))
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	m := createMap(segments)

	dangerCount := countDangerAreas(m)
	fmt.Println("Result:", dangerCount)
}

func createMap(segments []*Segment) [1000][1000]int {
	m := [1000][1000]int{}
	for _, s := range segments {
		x1 := s.x1
		x2 := s.x2
		y1 := s.y1
		y2 := s.y2
		if s.y1 > s.y2 {
			y1 = s.y2
			y2 = s.y1
		}
		if s.x1 > s.x2 {
			x1 = s.x2
			x2 = s.x1
		}
		if x1 == x2 || y1 == y2 {
			for y := y1; y <= y2; y++ {
				for x := x1; x <= x2; x++ {
					m[y][x]++
				}
			}
		}
	}
	return m
}

func dumpMap(m [1000][1000]int) {
	for x := 0; x < 1000; x++ {
		for y := 0; y < 1000; y++ {
			fmt.Printf("%d", m[x][y])
		}
		fmt.Println()
	}
	fmt.Println()
}

func countDangerAreas(m [1000][1000]int) int {
	count := 0
	for y := 0; y < 1000; y++ {
		for x := 0; x < 1000; x++ {
			if m[y][x] > 1 {
				count++
			}
		}
	}
	return count
}
