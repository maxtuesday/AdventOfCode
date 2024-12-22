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

func rev[T any](arr []T) {
	for i, j := 0, len(arr)-1; i < j; i, j = i+1, j-1 {
		arr[i], arr[j] = arr[j], arr[i]
	}
}

func transpose[T any](arr [][]T) {
	if len(arr) != len(arr[0]) {
		panic("only support transpose for NxN matrix")
	}

	for r := 0; r < len(arr); r++ {
		for c := r + 1; c < len(arr); c++ {
			arr[r][c], arr[c][r] = arr[c][r], arr[r][c]
		}
	}
}

func revRows[T any](arr [][]T) {
	for r := range arr {
		rev(arr[r])
	}
}

func revColumns[T any](arr [][]T) {
	for i, j := 0, len(arr)-1; i < j; i, j = i+1, j-1 {
		arr[i], arr[j] = arr[j], arr[i]
	}
}

func copyMap[K comparable, V any](m map[K]V) map[K]V {
	copy := make(map[K]V)
	for k, v := range m {
		copy[k] = v
	}
	return copy
}
