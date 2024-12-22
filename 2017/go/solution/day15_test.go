package solution

import "testing"

func TestDay15_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `Generator A starts with 65
Generator B starts with 8921`,
			expected: "588",
		},
	}

	for _, tt := range tts {
		actual := Day15{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}

func TestDay15_Part2(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `Generator A starts with 65
Generator B starts with 8921`,
			expected: "309",
		},
	}

	for _, tt := range tts {
		actual := Day15{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
