package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

const INPUT_FILE = "../../input/day15.txt"

var WIDTH int
var HEIGHT int

var SECTOR = [5][5]int{
	{0, 1, 2, 3, 4},
	{1, 2, 3, 4, 5},
	{2, 3, 4, 5, 6},
	{3, 4, 5, 6, 7},
	{4, 5, 6, 7, 8},
}

type Node struct {
	x, y int
}

// Priority Queue Impl from:
// https://pkg.go.dev/container/heap

// An Item is something we manage in a priority queue.
type Item struct {
	value    Node // The value of the item; arbitrary.
	priority int  // The priority of the item in the queue.
	// The index is needed by update and is maintained by the heap.Interface methods.
	index int // The index of the item in the heap.
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	// -- We want Pop to give us the highest, not lowest, priority so we use greater than here. --
	// We want Pop to give us the lowest, not highest, priority so we use less than here.
	return pq[i].priority < pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil  // avoid memory leak
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}

// update modifies the priority and value of an Item in the queue.
func (pq *PriorityQueue) update(item *Item, value Node, priority int) {
	item.value = value
	item.priority = priority
	heap.Fix(pq, item.index)
}

func GetNeighbors(n Node) []Node {
	neighbors := []Node{}
	down := Node{x: n.x, y: n.y + 1}
	up := Node{x: n.x, y: n.y - 1}
	right := Node{x: n.x + 1, y: n.y}
	left := Node{x: n.x - 1, y: n.y}

	if n.x > 0 {
		neighbors = append(neighbors, left)
	}
	if n.x < WIDTH {
		neighbors = append(neighbors, right)
	}

	if n.y > 0 {
		neighbors = append(neighbors, up)
	}
	if n.y < HEIGHT {
		neighbors = append(neighbors, down)
	}

	return neighbors
}

func FilterNeighbors(nodes []Node, queue map[Node]int) []Node {
	validNeighbors := []Node{}
	for _, n := range nodes {
		if _, ok := queue[n]; ok {
			validNeighbors = append(validNeighbors, n)
		}
	}
	return validNeighbors
}

func MinDist(dist map[Node]int) Node {
	min := math.MaxInt
	var minNode Node
	for n := range dist {
		if dist[n] < min {
			minNode = n
			min = dist[n]
		}
	}
	return minNode
}

func GetRisk(graph [][]int, x int, y int) int {
	sizeY := len(graph)
	sizeX := len(graph[0])
	sec_x := x / sizeX
	sec_y := y / sizeY

	factor := SECTOR[sec_y][sec_x]

	origin_x := x % sizeX
	origin_y := y % sizeY

	origin_risk := graph[origin_y][origin_x]

	sec_risk := (origin_risk + factor) % 9
	if sec_risk == 0 {
		sec_risk = 9
	}

	return sec_risk
}

func LoadData(filename string) [][]int {
	file, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	riskMap := [][]int{}
	for scanner.Scan() {
		row := []int{}
		for _, s := range strings.Split(scanner.Text(), "") {
			n, _ := strconv.Atoi(s)
			row = append(row, n)
		}
		riskMap = append(riskMap, row)
	}
	return riskMap
}

func Dump(arr [][]int) {
	for row := range arr {
		for col := range arr[row] {
			fmt.Printf("%4d", arr[row][col])
		}
		fmt.Println()
	}
	fmt.Println()
}

func DumpPQ(pq *PriorityQueue) {
	for i := 0; i < pq.Len(); i++ {
		fmt.Printf("%#v\n", heap.Pop(pq).(*Item))
	}
}

func Dijkstra(graph [][]int, source Node, target Node) int {
	dist := make(map[Node]int)
	queue := make(map[Node]int)

	items := make(map[Node]*Item)

	dist[source] = 0
	queue[source] = 0

	pq := make(PriorityQueue, 0)

	index := 0
	item := &Item{
		value:    source,
		priority: 0,
		index:    index,
	}
	heap.Push(&pq, item)
	items[source] = item
	index++

	// Init PQ
	for i := 0; i < HEIGHT; i++ {
		for j := 0; j < WIDTH; j++ {
			n := Node{x: j, y: i}
			if n != source {
				dist[n] = math.MaxInt
				queue[n] = math.MaxInt

				item := &Item{
					value:    n,
					priority: dist[n],
					index:    index,
				}
				heap.Push(&pq, item)
				items[n] = item
				index++
			}
		}
	}

	for pq.Len() != 0 {
		i := heap.Pop(&pq).(*Item)
		u := i.value
		delete(queue, u)

		if u == target {
			return dist[u]
		}

		neighbors := GetNeighbors(u)
		validNeighbors := FilterNeighbors(neighbors, queue)
		for _, v := range validNeighbors {
			alt := dist[u] + GetRisk(graph, v.x, v.y)
			if alt < dist[v] {
				dist[v] = alt
				pq.update(items[v], v, alt)
			}
		}

	}
	return -1
}

func main() {
	m := LoadData(INPUT_FILE)

	HEIGHT = len(m)
	WIDTH = len(m[0])

	src := Node{0, 0}
	target := Node{WIDTH - 1, HEIGHT - 1}
	res := Dijkstra(m, src, target)

	fmt.Println("Part 1:", res)

	HEIGHT = len(m) * 5
	WIDTH = len(m[0]) * 5
	target = Node{WIDTH - 1, HEIGHT - 1}
	res = Dijkstra(m, src, target)

	fmt.Println("Part 2:", res)
}
