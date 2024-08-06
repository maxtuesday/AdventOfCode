package solution

import "testing"

func TestDay01_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    "1122",
			expected: "3",
		},
		{
			input:    "1111",
			expected: "4",
		},
		{
			input:    "1234",
			expected: "0",
		},
		{
			input:    "91212129",
			expected: "9",
		},
	}

	for _, tt := range tts {
		actual := Day01{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
