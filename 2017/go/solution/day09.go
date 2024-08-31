package solution

import "fmt"

type Day09 struct {
}

type Stream struct {
	stream string
	index  int
}

// '{' starts a group when we are not in garbage
// '}' starts a group when we are not in garbage
// '!' skips the next character
func (s *Stream) process() int {
	score := 0
	for s.index < len(s.stream) {
		switch s.stream[s.index] {
		case '{':
			score += s.handleGroup(1)
		case '<':
			s.handleGarbage()
		case '!':
			s.index += 2
		default:
			s.index++
		}
	}
	return score
}

func (s *Stream) handleGroup(depth int) int {
	score := 0
	for s.index < len(s.stream) {
		s.index++
		switch s.stream[s.index] {
		case '{':
			score += s.handleGroup(depth + 1)
		case '}':
			return score + depth
		case '<':
			s.handleGarbage()
		case '!':
			s.index++
		}
	}
	return score + depth
}

func (s *Stream) handleGarbage() {
	for s.index < len(s.stream) {
		s.index++
		switch s.stream[s.index] {
		case '>':
			return
		case '!':
			s.index++
		}
	}
}

func (d Day09) Part1(input string) string {
	s := &Stream{
		stream: input,
		index:  0,
	}
	groups := s.process()
	return fmt.Sprintf("%d", groups)
}

func (d Day09) Part2(input string) string {
	return ""
}
