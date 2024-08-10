package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day07 struct {
}

type Disk struct {
	name     string
	weight   int
	children []string
}

func (d Day07) parse(input string) map[string]*Disk {
	lines := strings.Split(input, "\n")

	disks := map[string]*Disk{}
	for _, line := range lines {
		// Examples formats:
		// pbga (66)
		// fwft (72) -> ktlj, cntj, xhth

		tokens := strings.Fields(line)
		name := tokens[0]
		weight, _ := strconv.Atoi(strings.ReplaceAll(strings.ReplaceAll(tokens[1], "(", ""), ")", ""))
		children := []string{}
		if len(tokens) > 2 {
			children = strings.Fields(strings.ReplaceAll(strings.Join(tokens[3:], ""), ",", " "))
		}

		disks[name] = &Disk{name, weight, children}
	}
	return disks
}

func (d Day07) getRoot(disks map[string]*Disk) *Disk {
	// create reverse adj list
	rev := map[string]string{}
	for name, disk := range disks {
		for _, children := range disk.children {
			rev[children] = name
		}
	}

	for name := range disks {
		if _, ok := rev[name]; !ok {
			return disks[name]
		}
	}

	panic("could not find root")
}

func (d Day07) Part1(input string) string {
	disks := d.parse(input)
	root := d.getRoot(disks)
	return root.name
}

func dfs(disk *Disk, disks map[string]*Disk) (int, bool) {
	if len(disk.children) == 0 {
		return disk.weight, true
	}

	weights := []int{}
	for _, child := range disk.children {
		weight, isLayerOk := dfs(disks[child], disks)
		if !isLayerOk {
			return weight, isLayerOk
		}
		weights = append(weights, weight)
	}

	// check if all the weights are the same
	freqs := map[int]int{}
	for _, w := range weights {
		freqs[w]++
	}

	if len(freqs) != 1 {
		outlier := 0
		expected := 0
		for k, v := range freqs {
			if v == 1 {
				outlier = k
			} else {
				expected = k
			}
		}

		// which child?
		var d *Disk
		for i := range weights {
			if weights[i] == outlier {
				d = disks[disk.children[i]]
			}
		}

		if outlier > expected {
			return d.weight - abs(expected-outlier), false
		} else {
			return d.weight + abs(expected-outlier), false
		}
	}

	return weights[0]*len(weights) + disk.weight, true
}

func (d Day07) Part2(input string) string {
	disks := d.parse(input)
	root := d.getRoot(disks)
	w, _ := dfs(root, disks)
	return fmt.Sprintf("%d", w)
}
