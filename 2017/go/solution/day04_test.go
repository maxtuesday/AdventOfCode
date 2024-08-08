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
