package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day12 struct {
}

func (d Day12) dfs(cur int, adj map[int][]int, visited map[int]bool, total *int) {
	if visited[cur] {
		return
	}

	*total++
	visited[cur] = true

	for _, nei := range adj[cur] {
		d.dfs(nei, adj, visited, total)
	}
}

func (d Day12) dedup(arr []int) []int {
	set := map[int]struct{}{}
	for i := range arr {
		set[arr[i]] = struct{}{}
	}
	next := []int{}
	for k := range set {
		next = append(next, k)
	}
	return next
}

func (d Day12) parse(input string) map[int][]int {
	adj := map[int][]int{}

	lines := strings.Split(input, "\n")
	for _, line := range lines {
		tokens := strings.Fields(strings.ReplaceAll(line, ",", " "))

		src, err := strconv.Atoi(tokens[0])
		if err != nil {
			panic(err)
		}

		children := []int{}
		for _, n := range tokens[2:] {
			c, err := strconv.Atoi(n)
			if err != nil {
				panic(err)
			}
			children = append(children, c)
		}

		adj[src] = append(adj[src], children...)
		for _, child := range children {
			adj[child] = append(adj[child], src)
		}
	}

	for k, v := range adj {
		adj[k] = d.dedup(v)
	}

	return adj
}

func (d Day12) Part1(input string) string {
	adj := d.parse(input)

	total := 0
	visited := map[int]bool{}
	d.dfs(0, adj, visited, &total)

	return fmt.Sprintf("%d", total)
}

func (d Day12) Part2(input string) string {
	adj := d.parse(input)

	N := len(adj)
	visited := map[int]bool{}

	groups := 0
	for i := 0; i < N; i++ {
		total := 0
		d.dfs(i, adj, visited, &total)
		if total > 0 {
			groups++
		}
	}

	return fmt.Sprintf("%d", groups)
}
