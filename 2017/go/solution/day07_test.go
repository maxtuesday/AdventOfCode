package solution

import "testing"

func TestDay07_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)`,
			expected: "tknk",
		},
	}

	for _, tt := range tts {
		actual := Day07{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}

func TestDay07_Part2(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)`,
			expected: "60",
		},
	}

	for _, tt := range tts {
		actual := Day07{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
