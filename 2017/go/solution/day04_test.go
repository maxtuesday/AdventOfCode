package solution

import "testing"

func TestDay04_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    `aa bb cc dd ee`,
			expected: "1",
		},
		{
			input:    `aa bb cc dd aa`,
			expected: "0",
		},
		{
			input:    `aa bb cc dd aaa`,
			expected: "1",
		},
	}

	for _, tt := range tts {
		actual := Day04{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}

func TestDay04_Part2(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    `abcde fghij`,
			expected: "1",
		},
		{
			input:    `abcde xyz ecdab`,
			expected: "0",
		},
		{
			input:    `a ab abc abd abf abj`,
			expected: "1",
		},
		{
			input:    `iiii oiii ooii oooi oooo`,
			expected: "1",
		},
		{
			input:    `oiii ioii iioi iiio`,
			expected: "0",
		},
	}

	for _, tt := range tts {
		actual := Day04{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
