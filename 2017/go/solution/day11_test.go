package solution

import "testing"

func TestDay11_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    "ne,ne,ne",
			expected: "3",
		},
		{
			input:    "ne,ne,sw,sw",
			expected: "0",
		},
		{
			input:    "ne,ne,s,s",
			expected: "2",
		},
		{
			input:    "se,sw,se,sw,sw",
			expected: "3",
		},
	}

	for _, tt := range tts {
		actual := Day11{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
