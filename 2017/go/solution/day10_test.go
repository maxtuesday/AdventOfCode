package solution

import "testing"

func TestDay10_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    "3, 4, 1, 5",
			expected: "12",
		},
	}

	for _, tt := range tts {
		actual := Day10{5}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
