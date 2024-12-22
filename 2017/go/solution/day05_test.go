package solution

import "testing"

func TestDay05_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `0
3
0
1
-3`,
			expected: "5",
		},
	}

	for _, tt := range tts {
		actual := Day05{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}

func TestDay05_Part2(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `0
3
0
1
-3`,
			expected: "10",
		},
	}

	for _, tt := range tts {
		actual := Day05{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
