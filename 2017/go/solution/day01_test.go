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

func TestDay01_Part2(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    "1212",
			expected: "6",
		},
		{
			input:    "1221",
			expected: "0",
		},
		{
			input:    "123425",
			expected: "4",
		},
		{
			input:    "123123",
			expected: "12",
		},
		{
			input:    "12131415",
			expected: "4",
		},
	}

	for _, tt := range tts {
		actual := Day01{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
