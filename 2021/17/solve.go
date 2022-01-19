package main

import (
	"fmt"
)

type TargetArea struct {
	x1, x2, y1, y2 int
}

func main() {
	// sampleInput := "target area: x=20..30, y=-10..-5"
	// ta := TargetArea{20, 30, -10, -5}
	// input := "target area: x=124..174, y=-123..-86"
	ta := TargetArea{124, 174, -123, -86}
	Part1(ta)
	Part2(ta)
}

func next_x(pos, vel int) (new_pos int, new_vel int) {
	pos += vel
	if vel > 0 {
		vel--
	}
	if vel < 0 {
		vel++
	}
	return pos, vel
}

func find_x(ta TargetArea, max_x_vel int) []int {
	vels := []int{}
	for xv := 1; xv <= max_x_vel; xv++ {
		x := 0
		vel := xv
		for step := 1; ; step++ {
			x, vel = next_x(x, vel)
			if x >= ta.x1 && x <= ta.x2 {
				vels = append(vels, xv)
				break
			}
			if vel == 0 {
				if x >= ta.x1 && x <= ta.x2 {
					vels = append(vels, xv)
					break
				}
				if x < ta.x1 || x > ta.x2 {
					break
				}
			}
		}
	}
	return vels
}

func next_y(pos, vel int) (new_pos int, new_vel int) {
	pos += vel
	vel--
	return pos, vel
}

func find_y(ta TargetArea, min_y_vel int, max_y_vel int) []int {
	vels := []int{}
	for yv := min_y_vel; yv <= max_y_vel; yv++ {
		y := 0
		vel := yv
		for step := 1; ; step++ {
			y, vel = next_y(y, vel)
			if y >= ta.y1 && y <= ta.y2 {
				vels = append(vels, yv)
				break
			}
			if y < ta.y2 && y < ta.y1 {
				break
			}
		}
	}
	return vels
}

func find_y_pos(vel int, ta TargetArea) int {
	y := 0
	max_y := 0
	for step := 0; ; step++ {
		y, vel = next_y(y, vel)
		if max_y < y {
			max_y = y
		}
		if y >= ta.y1 && y <= ta.y2 {
			return max_y
		}
	}
}

func Part1(ta TargetArea) {
	yvs := find_y(ta, ta.y2, -1*ta.y1-1)
	max_y := find_y_pos(yvs[len(yvs)-1], ta)
	fmt.Printf("Part 1: %d\n", max_y)
}

func lands_in_target(ta TargetArea, xv int, yv int) bool {
	x, y := 0, 0
	for step := 1; ; step++ {
		x, xv = next_x(x, xv)
		y, yv = next_y(y, yv)
		if x >= ta.x1 && x <= ta.x2 &&
			y >= ta.y1 && y <= ta.y2 {
			return true
		}
		if y < ta.y2 && y < ta.y1 {
			return false
		}
	}
}

func Part2(ta TargetArea) {
	xvs := find_x(ta, ta.x2)
	yvs := find_y(ta, ta.y1, -1*ta.y1-1)
	valid := 0
	for _, xv := range xvs {
		for _, yv := range yvs {
			if lands_in_target(ta, xv, yv) {
				valid++
			}
		}
	}
	fmt.Println("Part 2:", valid)
}
