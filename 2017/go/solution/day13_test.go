package solution

import "testing"

func TestDay13_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `0: 3
1: 2
4: 4
6: 4`,
			expected: "24",
		},
	}

	for _, tt := range tts {
		actual := Day13{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}

func TestDay13_Part2(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `0: 3
1: 2
4: 4
6: 4`,
			expected: "10",
		},
	}

	for _, tt := range tts {
		actual := Day13{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
