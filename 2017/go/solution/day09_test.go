package solution

import "testing"

func TestDay09_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    "{}",
			expected: "1",
		},
		{
			input:    "{{{}}}",
			expected: "6",
		},
		{
			input:    "{{},{}}",
			expected: "5",
		},
		{
			input:    "{{{},{},{{}}}}",
			expected: "16",
		},
		{
			input:    "{<a>,<a>,<a>,<a>}",
			expected: "1",
		},
		{
			input:    "{{<ab>},{<ab>},{<ab>},{<ab>}}",
			expected: "9",
		},
		{
			input:    "{{<!!>},{<!!>},{<!!>},{<!!>}}",
			expected: "9",
		},
		{
			input:    "{{<a!>},{<a!>},{<a!>},{<ab>}}",
			expected: "3",
		},
	}

	for _, tt := range tts {
		actual := Day09{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}

func TestDay09_Part2(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input:    "<>",
			expected: "0",
		},
		{
			input:    "<random characters>",
			expected: "17",
		},
		{
			input:    "<<<<>",
			expected: "3",
		},
		{
			input:    "<{!>}>",
			expected: "2",
		},
		{
			input:    "<!!>",
			expected: "0",
		},
		{
			input:    "<!!!>>",
			expected: "0",
		},
		{
			input:    "<{o\"i!a,<{i<a>",
			expected: "10",
		},
	}

	for _, tt := range tts {
		actual := Day09{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
