package solution

import "testing"

func TestDay17_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    "3",
			expected: "638",
		},
	}

	for _, tt := range tts {
		actual := Day17{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
