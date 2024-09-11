package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day13 struct {
}

type Layer struct {
	depth            int
	scannerRange     int
	scannerPos       int
	scannerDirection int
}

func (l *Layer) Step() {
	if l.scannerPos == 0 {
		l.scannerDirection = 1
	} else if l.scannerPos == l.scannerRange-1 {
		l.scannerDirection = -1
	}
	l.scannerPos += l.scannerDirection
}

func (l *Layer) Severity() int {
	return l.depth * l.scannerRange
}

type Firewall struct {
	layers map[int]*Layer
}

func (f *Firewall) Step() {
	for _, v := range f.layers {
		v.Step()
	}
}

func (d Day13) Part1(input string) string {
	// layer: depth
	lines := strings.Split(input, "\n")

	layers := map[int]*Layer{}
	maxLayers := 0
	for _, line := range lines {
		tokens := strings.Split(line, ":")
		d, _ := strconv.Atoi(strings.TrimSpace(tokens[0]))
		r, _ := strconv.Atoi(strings.TrimSpace(tokens[1]))

		layers[d] = &Layer{
			depth:            d,
			scannerRange:     r,
			scannerDirection: 1,
		}
		maxLayers = max(maxLayers, d)
	}

	firewall := &Firewall{
		layers: layers,
	}

	// simulate
	severity := 0
	for i := 0; i <= maxLayers; i++ {
		// check if pos is same space as scanner
		if l, ok := firewall.layers[i]; ok && l.scannerPos == 0 {
			// caught
			severity += firewall.layers[i].Severity()
		}

		// move
		firewall.Step()
	}

	return fmt.Sprintf("%d", severity)
}

func (d Day13) Part2(input string) string {
	return ""
}
