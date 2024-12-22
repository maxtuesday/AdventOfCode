package solution

import "fmt"

type Day09 struct {
}

type Stream struct {
	stream string
	index  int
}

// '{' starts a group when we are not in garbage
// '}' ends a group when we are not in garbage
// '<' starts garbage
// '>' end garbage
// '!' skips the next character
func (s *Stream) process() (int, int) {
	score := 0
	removedGarbage := 0
	for s.index < len(s.stream) {
		switch s.stream[s.index] {
		case '{':
			s, r := s.handleGroup(1)
			score += s
			removedGarbage += r
		case '<':
			removedGarbage += s.handleGarbage()
		case '!':
			s.index += 2
		default:
			s.index++
		}
	}
	return score, removedGarbage
}

func (s *Stream) handleGroup(depth int) (int, int) {
	score := 0
	removedGarbage := 0
	for s.index < len(s.stream) {
		s.index++
		switch s.stream[s.index] {
		case '{':
			s, r := s.handleGroup(depth + 1)
			score += s
			removedGarbage += r
		case '}':
			return score + depth, removedGarbage
		case '<':
			removedGarbage += s.handleGarbage()
		case '!':
			s.index++
		}
	}
	panic("unreachable")
}

func (s *Stream) handleGarbage() int {
	chars := 0
	for s.index < len(s.stream) {
		s.index++
		switch s.stream[s.index] {
		case '>':
			return chars
		case '!':
			s.index++
		default:
			chars++
		}
	}
	panic("unreachable")
}

func (d Day09) Part1(input string) string {
	s := &Stream{
		stream: input,
		index:  0,
	}
	score, _ := s.process()
	return fmt.Sprintf("%d", score)
}

func (d Day09) Part2(input string) string {
	s := &Stream{
		stream: input,
		index:  0,
	}
	_, removedGarbage := s.process()
	return fmt.Sprintf("%d", removedGarbage)
}
