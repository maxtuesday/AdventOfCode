package solution

import "testing"

func TestDay03_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    `1`,
			expected: "0",
		},
		{
			input:    `12`,
			expected: "3",
		},
		{
			input:    `23`,
			expected: "2",
		},
		{
			input:    `1024`,
			expected: "31",
		},
	}

	for _, tt := range tts {
		actual := Day03{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
