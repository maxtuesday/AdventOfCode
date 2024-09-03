package solution

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func copyMap[K comparable, V any](m map[K]V) map[K]V {
	copy := make(map[K]V)
	for k, v := range m {
		copy[k] = v
	}
	return copy
}
