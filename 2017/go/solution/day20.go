package solution

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Day20 struct {
}

type Coord struct {
	x, y, z int
}

type Particle struct {
	pos Coord
	vel Coord
	acc Coord
}

func parseCoords(component string) Coord {
	tokens := strings.Split(component[3:len(component)-1], ",")
	nums := []int{}
	for _, token := range tokens {
		n, _ := strconv.Atoi(strings.TrimSpace(token))
		nums = append(nums, n)
	}
	return Coord{
		x: nums[0],
		y: nums[1],
		z: nums[2],
	}
}

func NewParticle(line string) Particle {
	parts := strings.Split(line, ", ")
	return Particle{
		pos: parseCoords(parts[0]),
		vel: parseCoords(parts[1]),
		acc: parseCoords(parts[2]),
	}
}

func (d Day20) parse(input string) []Particle {
	particles := []Particle{}
	for _, line := range strings.Split(input, "\n") {
		particles = append(particles, NewParticle(line))
	}
	return particles
}

func (d Day20) updateParticles(particles []Particle) {
	for i := range particles {
		// Increase the X velocity by the X acceleration.
		particles[i].vel.x += particles[i].acc.x
		// Increase the Y velocity by the Y acceleration.
		particles[i].vel.y += particles[i].acc.y
		// Increase the Z velocity by the Z acceleration.
		particles[i].vel.z += particles[i].acc.z
		// Increase the X position by the X velocity.
		particles[i].pos.x += particles[i].vel.x
		// Increase the Y position by the Y velocity.
		particles[i].pos.y += particles[i].vel.y
		// Increase the Z position by the Z velocity.
		particles[i].pos.z += particles[i].vel.z
	}
}

func (d Day20) findClosest(particles []Particle) int {
	minDist := math.MaxInt
	idx := 0
	for i, p := range particles {
		dist := abs(p.pos.x) + abs(p.pos.y) + abs(p.pos.z)
		if dist < minDist {
			minDist = dist
			idx = i
		}
	}
	return idx
}

func (d Day20) Part1(input string) string {
	particles := d.parse(input)

	for i := 0; i < 500; i++ {
		d.updateParticles(particles)
	}

	return fmt.Sprintf("%d", d.findClosest(particles))
}

func (d Day20) removeCollisions(particles []Particle) []Particle {
	positions := map[Coord][]Particle{}
	for _, p := range particles {
		positions[p.pos] = append(positions[p.pos], p)
	}
	p := []Particle{}
	for _, v := range positions {
		if len(v) == 1 {
			p = append(p, v[0])
		}
	}
	return p
}

func (d Day20) Part2(input string) string {
	particles := d.parse(input)

	for i := 0; i < 50; i++ {
		d.updateParticles(particles)
		particles = d.removeCollisions(particles)
	}

	return fmt.Sprintf("%d", len(particles))
}
