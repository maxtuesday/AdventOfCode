package solution

import "testing"

func TestDay02_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `5 1 9 5
7 5 3
2 4 6 8`,
			expected: "18",
		},
	}

	for _, tt := range tts {
		actual := Day02{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}

func TestDay02_Part2(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `5 9 2 8
9 4 7 3
3 8 6 5`,
			expected: "9",
		},
	}

	for _, tt := range tts {
		actual := Day02{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
