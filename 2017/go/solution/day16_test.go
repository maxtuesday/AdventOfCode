package solution

import "testing"

func TestDay16_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    "s1",
			expected: "eabcd",
		},
		{
			input:    "x3/4",
			expected: "abced",
		},
		{
			input:    "pe/b",
			expected: "aecdb",
		},
		{
			input:    "s1,x3/4,pe/b",
			expected: "baedc",
		},
	}

	for _, tt := range tts {
		actual := Day16{"abcde"}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
