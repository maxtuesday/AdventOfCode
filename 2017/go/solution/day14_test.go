package solution

import "testing"

func TestDay14_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    "flqrgnkx",
			expected: "8108",
		},
	}

	for _, tt := range tts {
		actual := Day14{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
