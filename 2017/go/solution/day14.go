package solution

import (
	"fmt"
	"strconv"
)

type Day14 struct {
}

type DiskDefrag struct {
	Grid [][]bool
}

func NewDiskDefrad(input string) DiskDefrag {
	hashes := make([]string, 128)
	for i := 0; i < len(hashes); i++ {
		hashes[i] = KnotHash(fmt.Sprintf("%s-%d", input, i))
	}

	// convert the hex hash into 128 bit binary
	grid := make([][]bool, 128)
	for i := range grid {
		grid[i] = make([]bool, 128)
	}

	for r := range grid {
		row := ""
		for i := range hashes[r] {
			ch := hashes[r][i]
			n, _ := strconv.ParseInt(string(ch), 16, 0)
			bin := fmt.Sprintf("%04b", n)
			row += bin
		}
		for i := range row {
			used := false
			if row[i] == '1' {
				used = true
			}
			grid[r][i] = used
		}
	}

	return DiskDefrag{
		Grid: grid,
	}
}

func (d DiskDefrag) CountUsed() int {
	used := 0
	for r := range d.Grid {
		for c := range d.Grid[r] {
			if d.Grid[r][c] {
				used++
			}
		}
	}
	return used
}

func (d DiskDefrag) CountRegions() int {
	DR := []int{1, 0, -1, 0}
	DC := []int{0, 1, 0, -1}

	isOOB := func(pos [2]int) bool {
		return pos[0] < 0 || pos[0] >= len(d.Grid) || pos[1] < 0 || pos[1] >= len(d.Grid[0])
	}

	var dfs func(pos [2]int, visited map[[2]int]struct{})
	dfs = func(pos [2]int, visited map[[2]int]struct{}) {
		if _, ok := visited[pos]; ok {
			return
		}

		visited[pos] = struct{}{}
		for i := range DR {
			next := [2]int{
				pos[0] + DR[i],
				pos[1] + DC[i],
			}
			if !isOOB(next) && d.Grid[next[0]][next[1]] {
				dfs(next, visited)
			}
		}
	}

	regions := 0
	visited := map[[2]int]struct{}{}
	for r := range d.Grid {
		for c := range d.Grid[r] {
			pos := [2]int{r, c}
			if _, ok := visited[pos]; !ok && d.Grid[pos[0]][pos[1]] {
				dfs(pos, visited)
				regions++
			}
		}
	}

	return regions
}

func (d Day14) Part1(input string) string {
	diskDefrag := NewDiskDefrad(input)
	used := diskDefrag.CountUsed()
	return fmt.Sprintf("%d", used)
}

func (d Day14) Part2(input string) string {
	diskDefrag := NewDiskDefrad(input)
	regions := diskDefrag.CountRegions()
	return fmt.Sprintf("%d", regions)
}
