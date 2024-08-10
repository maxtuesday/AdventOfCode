package solution

import (
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

func (d Day07) Part1(input string) string {
	disks := d.parse(input)

	// create reverse adj list
	rev := map[string]string{}
	for name, disk := range disks {
		for _, children := range disk.children {
			rev[children] = name
		}
	}

	for name := range disks {
		if _, ok := rev[name]; !ok {
			return name
		}
	}

	return "not found!"
}

func (d Day07) Part2(input string) string {
	return ""
}
