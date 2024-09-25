package solution

import "testing"

func TestDay20_Part1(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>`,
			expected: "0",
		},
	}

	for _, tt := range tts {
		actual := Day20{}.Part1(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}

func TestDay20_Part2(t *testing.T) {
	type TestCase struct {
		input    string
		expected string
	}

	tts := []TestCase{
		{
			input: `p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>`,
			expected: "1",
		},
	}

	for _, tt := range tts {
		actual := Day20{}.Part2(tt.input)
		if actual != tt.expected {
			t.Errorf("Expected: %s; Got: %s", tt.expected, actual)
		}
	}
}
