package solution

import "testing"

func TestDay22_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `..#
#..
...`,
			expected: "5587",
		},
	}

	for _, tt := range tts {
		actual := Day22{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}

func TestDay22_Part2(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `..#
#..
...`,
			expected: "2511944",
		},
	}

	for _, tt := range tts {
		actual := Day22{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
