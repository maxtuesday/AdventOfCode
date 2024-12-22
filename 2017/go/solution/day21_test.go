package solution

import "testing"

func TestDay21_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#`,
			expected: "12",
		},
	}

	for _, tt := range tts {
		actual := Day21{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
