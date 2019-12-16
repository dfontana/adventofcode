package main

import (
	"fmt"

	"github.com/dfontana/adventofcode2019/intcode"
)

const (
	North = 1
	South = 2
	West  = 3
	East  = 4

	Wall   = 0
	Open   = 1
	Oxygen = 2
)

var (
	Opposite   = map[int64]int64{North: South, South: North, East: West, West: East}
	Directions = map[int64]Coord{
		North: Coord{0, -1},
		East:  Coord{1, 0},
		West:  Coord{-1, 0},
		South: Coord{0, 1},
	}
)

type Coord struct {
	x int
	y int
}

type DistCoord struct {
	Coord
	steps int
}

type PathCoord struct {
	Coord
	prior      *PathCoord
	dirToPrior int64
}

func (c Coord) Add(o Coord) Coord {
	return Coord{c.x + o.x, c.y + o.y}
}

func main() {
	data := intcode.ReadProgram("./input.txt")
	conf := intcode.Config().SendRequest()
	go intcode.Run(data, conf)

	res, space := search(conf.Input, conf.Output, conf.Request)
	fmt.Println("Part 1: Oxygen at", *res)

	res2 := reverseFill(space, res.Coord)
	fmt.Println("Part 2: Time to fill", res2)
}

func search(in chan<- int64, out <-chan int64, req <-chan bool) (goal *DistCoord, space map[Coord]int) {
	botLoc := Coord{0, 0}
	space = make(map[Coord]int)
	space[botLoc] = Open

	moveTo := func(dest DistCoord) Coord {
		var ptr *PathCoord
		mFront := []PathCoord{PathCoord{dest.Coord, nil, -1}}
		mSeen := map[Coord]bool{dest.Coord: true}

		for len(mFront) != 0 {
			mTarg := mFront[0]
			mFront = mFront[1:]
			if mTarg.Coord == botLoc {
				ptr = &mTarg
				break
			}
			for dir, vec := range Directions {
				mNext := mTarg.Coord.Add(vec)
				if !mSeen[mNext] && space[mNext] == Open {
					mSeen[mNext] = true
					mFront = append(mFront, PathCoord{mNext, &mTarg, Opposite[dir]})
				}
			}
		}

		for ptr.prior != nil {
			<-req
			in <- ptr.dirToPrior
			<-out
			ptr = ptr.prior
		}
		return dest.Coord
	}

	var front DistCoord
	frontier := []DistCoord{DistCoord{botLoc, 0}}
	for len(frontier) != 0 {
		// Pop and move bot there to resume search
		front, frontier = frontier[0], frontier[1:]
		botLoc = moveTo(front)
		for dir, vec := range Directions {
			next := botLoc.Add(vec)
			nextDist := DistCoord{next, front.steps + 1}
			if _, ok := space[next]; !ok {
				// Move the bot forwards
				<-req
				in <- dir
				switch <-out {
				case Wall:
					space[next] = Wall
				case Oxygen:
					if goal == nil {
						goal = &nextDist
					}
					fallthrough
				case Open:
					space[next] = Open
					frontier = append(frontier, nextDist)
					<-req
					in <- Opposite[dir]
					<-out
				}
			}
		}
	}
	return
}

func reverseFill(space map[Coord]int, start Coord) int {
	frontier := []DistCoord{DistCoord{start, 0}}
	explored := map[Coord]bool{start: true}

	maxDistToWall := 0
	var current DistCoord
	for len(frontier) != 0 {
		current, frontier = frontier[0], frontier[1:]

		if current.steps > maxDistToWall {
			maxDistToWall = current.steps
		}

		for _, vec := range Directions {
			next := current.Coord.Add(vec)
			if !explored[next] && space[next] == Open {
				explored[next] = true
				frontier = append(frontier, DistCoord{next, current.steps + 1})
			}
		}
	}
	return maxDistToWall
}
