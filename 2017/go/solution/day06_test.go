package solution

import "testing"

func TestDay06_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    `0 2 7 0`,
			expected: "5",
		},
	}

	for _, tt := range tts {
		actual := Day06{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}

func TestDay06_Part2(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    `0 2 7 0`,
			expected: "4",
		},
	}

	for _, tt := range tts {
		actual := Day06{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
