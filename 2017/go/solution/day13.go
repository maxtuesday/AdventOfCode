package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day13 struct {
}

type Layer struct {
	Depth int
	Range int
}

func (d Day13) Parse(input string) []Layer {
	lines := strings.Split(input, "\n")

	layers := make([]Layer, len(lines))
	for i := range lines {
		tokens := strings.Split(lines[i], ": ")
		d, err := strconv.Atoi(tokens[0])
		if err != nil {
			panic(err)
		}
		r, err := strconv.Atoi(tokens[1])
		if err != nil {
			panic(err)
		}

		layers[i] = Layer{
			Depth: d,
			Range: r,
		}
	}

	return layers
}

func (d Day13) Caught(time, rangeLen int) bool {
	return time%((rangeLen-1)*2) == 0
}

func (d Day13) Part1(input string) string {
	layers := d.Parse(input)

	severity := 0
	for _, layer := range layers {
		if d.Caught(layer.Depth, layer.Range) {
			severity += layer.Depth * layer.Range
		}
	}

	return fmt.Sprintf("%d", severity)
}

func (d Day13) Part2(input string) string {
	return ""
}
