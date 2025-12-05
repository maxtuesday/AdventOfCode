package solution

import "testing"

func TestDay01_Part1(t *testing.T) {
	input := `L68
L30
R48
L5
R60
L55
L1
L99
R14
L82`

	tts := []TestCase{
		{
			input:    input,
			expected: "3",
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
	input := `L68
L30
R48
L5
R60
L55
L1
L99
R14
L82`

	tts := []TestCase{
		{
			input:    input,
			expected: "6",
		},
	}

	for _, tt := range tts {
		actual := Day01{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
