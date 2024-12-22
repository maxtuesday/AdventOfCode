package solution

import (
	"fmt"
	"math"
	"strings"
)

type Day21 struct {
}

var start = [][]byte{
	{'.', '#', '.'},
	{'.', '.', '#'},
	{'#', '#', '#'},
}

type Pattern struct {
	pattern [][]byte
}

func NewPattern(p [][]byte) Pattern {
	return Pattern{p}
}

func FromString(pattern string) Pattern {
	rows := strings.Split(pattern, "/")
	p := make([][]byte, len(rows))
	for r := range rows {
		p[r] = append([]byte{}, rows[r]...)
	}
	return NewPattern(p)
}

func (p Pattern) Size() int {
	return len(p.pattern)
}

func (p Pattern) Copy() Pattern {
	pattern := make([][]byte, len(p.pattern))
	for i := range p.pattern {
		pattern[i] = make([]byte, len(p.pattern[i]))
		copy(pattern[i], p.pattern[i])
	}
	return NewPattern(pattern)
}

func (p Pattern) String() string {
	b := strings.Builder{}
	for i := range p.pattern {
		for j := range p.pattern[i] {
			b.WriteByte(p.pattern[i][j])
		}
		if i < len(p.pattern)-1 {
			b.WriteByte('/')
		}
	}
	return b.String()
}

func (p Pattern) Print() {
	fmt.Printf("Size: %d x %d\n", len(p.pattern), len(p.pattern[0]))
	for i := range p.pattern {
		for j := range p.pattern[i] {
			fmt.Printf("%c", p.pattern[i][j])
		}
		fmt.Println()
	}
	fmt.Println()
}

func (p Pattern) CountOn() int {
	on := 0
	for i := range p.pattern {
		for j := range p.pattern[i] {
			if p.pattern[i][j] == '#' {
				on++
			}
		}
	}
	return on
}

func (p Pattern) Rotate90() {
	transpose(p.pattern)
	revRows(p.pattern)
}

func (p Pattern) Rotate180() {
	p.Rotate90()
	p.Rotate90()
}

func (p Pattern) Rotate270() {
	transpose(p.pattern)
	revColumns(p.pattern)
}

func (p Pattern) Rotations() []Pattern {
	r90 := p.Copy()
	r90.Rotate90()

	r180 := p.Copy()
	r180.Rotate180()

	r270 := p.Copy()
	r270.Rotate270()
	return []Pattern{r90, r180, r270}
}

func (p Pattern) GetTransformations() []Pattern {
	tranformations := []Pattern{p}

	// flip on both axises
	x := p.Copy()
	revColumns(x.pattern)
	tranformations = append(tranformations, x)

	y := p.Copy()
	revRows(y.pattern)
	tranformations = append(tranformations, y)

	// rotate
	rotations := p.Rotations()
	tranformations = append(tranformations, rotations...)
	tranformations = append(tranformations, x.Rotations()...)
	tranformations = append(tranformations, y.Rotations()...)

	return tranformations
}

func (p Pattern) Break(size int) []Pattern {
	patterns := []Pattern{}
	s := p.Size()
	for r := 0; r < s; r += size {
		for c := 0; c < s; c += size {
			newPattern := [][]byte{
				p.pattern[r][c : c+size],
				p.pattern[r+1][c : c+size],
			}
			if size == 3 {
				newPattern = append(newPattern, p.pattern[r+2][c:c+size])
			}
			patterns = append(patterns, NewPattern(newPattern))
		}
	}
	return patterns
}

func (d Day21) Merge(patterns []Pattern) Pattern {
	// how many rows and columns?
	chunkSize := patterns[0].Size()
	size := int(math.Sqrt(float64(len(patterns))))
	pp := make([][]Pattern, size)
	offset := 0
	for r := 0; r < size; r++ {
		pp[r] = make([]Pattern, size)
		for c := 0; c < size; c++ {
			pp[r][c] = patterns[offset]
			offset++
		}
	}

	newPatternSize := chunkSize * size
	newPattern := make([][]byte, newPatternSize)
	for r := 0; r < newPatternSize; r++ {
		newPattern[r] = make([]byte, newPatternSize)
	}

	for r := 0; r < size; r++ {
		for c := 0; c < size; c++ {
			// initialize quadrant top left corner (r, c)
			for i := 0; i < chunkSize; i++ {
				for j := 0; j < chunkSize; j++ {
					foo := pp[r][c].pattern[i][j]
					newPattern[i+r*chunkSize][j+c*chunkSize] = foo
				}
			}
		}
	}

	return NewPattern(newPattern)
}

func (d Day21) process(p Pattern, rules map[string]string) Pattern {
	size := p.Size()
	var chunks []Pattern

	if size%2 == 0 {
		// break pixels up in 2x2 squares
		chunks = p.Break(2)
	} else if size%3 == 0 {
		// break pixels up in 3x3 squares
		chunks = p.Break(3)
	} else {
		panic("not divisible by 2 or 3")
	}

	nextPatterns := []Pattern{}
	for _, chunk := range chunks {
	_chunk:
		for _, t := range chunk.GetTransformations() {
			if next, ok := rules[t.String()]; ok {
				nextPatterns = append(nextPatterns, FromString(next))
				break _chunk
			}
		}
	}

	if len(nextPatterns) == 1 {
		return nextPatterns[0]
	}

	return d.Merge(nextPatterns)
}

func (d Day21) parse(input string) map[string]string {
	rules := map[string]string{}
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		tokens := strings.Split(line, " => ")
		rules[tokens[0]] = tokens[1]
	}
	return rules
}

func (d Day21) Part1(input string) string {
	rules := d.parse(input)

	p := NewPattern(start)
	for i := 0; i < 5; i++ {
		p = d.process(p, rules)
	}

	return fmt.Sprintf("%d", p.CountOn())
}

func (d Day21) Part2(input string) string {
	rules := d.parse(input)

	p := NewPattern(start)
	for i := 0; i < 18; i++ {
		p = d.process(p, rules)
	}

	return fmt.Sprintf("%d", p.CountOn())
}
