package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

const N = 1000

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

func createMap(segments []*Segment) [N][N]int {
	m := [N][N]int{}
	for _, s := range segments {
		// Find points between
		yp := []int{}
		xp := []int{}

		if s.y1 <= s.y2 {
			for y := s.y1; y <= s.y2; y++ {
				yp = append(yp, y)
			}
		} else {
			for y := s.y1; y >= s.y2; y-- {
				yp = append(yp, y)
			}
		}

		if s.x1 <= s.x2 {
			for x := s.x1; x <= s.x2; x++ {
				xp = append(xp, x)
			}
		} else {
			for x := s.x1; x >= s.x2; x-- {
				xp = append(xp, x)
			}
		}

		// Which has max points?
		if len(xp) > len(yp) {
			// yp should only have 1 point
			for i := 0; i < len(xp); i++ {
				m[yp[0]][xp[i]]++
			}
		} else if len(xp) < len(yp) {
			// xp should only have 1 point
			for i := 0; i < len(yp); i++ {
				m[yp[i]][xp[0]]++
			}
		} else {
			// both have same size
			for i := 0; i < len(xp); i++ {
				m[yp[i]][xp[i]]++
			}
		}

	}
	return m
}

func dumpMap(m [N][N]int) {
	for x := 0; x < N; x++ {
		for y := 0; y < N; y++ {
			n := m[x][y]
			if n == 0 {
				fmt.Print(".")
			} else {
				fmt.Printf("%d", n)
			}
		}
		fmt.Println()
	}
	fmt.Println()
}

func countDangerAreas(m [N][N]int) int {
	count := 0
	for y := 0; y < N; y++ {
		for x := 0; x < N; x++ {
			if m[y][x] > 1 {
				count++
			}
		}
	}
	return count
}

func calcSlope(x1, x2, y1, y2 int) (int, error) {
	dy := y2 - y1
	dx := x2 - x1

	if dx == 0 {
		return 0, errors.New("cannot divide by zero")
	} else {
		return dy / dx, nil
	}
}
