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

func TestDay10_Part2(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    "",
			expected: "a2582a3a0e66e6e86e3812dcb672a272",
		},
		{
			input:    "AoC 2017",
			expected: "33efeb34ea91902bb2f59c9920caa6cd",
		},
		{
			input:    "1,2,3",
			expected: "3efbe78a8d82f29979031a4aa0b16a9d",
		},
		{
			input:    "1,2,4",
			expected: "63960835bcdc130f0b66d7ff4f6a5a8e",
		},
	}

	for _, tt := range tts {
		actual := Day10{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
