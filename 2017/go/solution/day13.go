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
	layers    map[int]*Layer
	maxLayers int
}

func NewFirewall(input string) *Firewall {
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

	return &Firewall{
		layers:    layers,
		maxLayers: maxLayers,
	}
}

func (f *Firewall) Step() {
	for _, v := range f.layers {
		v.Step()
	}
}

func (f *Firewall) Severity() (int, bool) {
	severity := 0
	caught := false
	for i := 0; i <= f.maxLayers; i++ {
		// check if pos is same space as scanner
		if l, ok := f.layers[i]; ok && l.scannerPos == 0 {
			// caught
			caught = true
			severity += f.layers[i].Severity()
		}

		// move
		f.Step()
	}
	return severity, caught
}

func (d Day13) Part1(input string) string {
	firewall := NewFirewall(input)
	sev, _ := firewall.Severity()
	return fmt.Sprintf("%d", sev)
}

func (d Day13) Part2(input string) string {
	delay := 0
	for {
		firewall := NewFirewall(input)
		for i := 0; i < delay; i++ {
			firewall.Step()
		}
		if _, caught := firewall.Severity(); !caught {
			return fmt.Sprintf("%d", delay)
		}
		delay++
	}
}
