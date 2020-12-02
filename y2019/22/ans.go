package main

import (
	"fmt"
	"time"

	"github.com/dfontana/adventofcode/2019/util"
)

const (
	DEAL_INC   = "dealInc"
	DEAL_STACK = "dealStack"
	CUT        = "cut"
)

type command struct {
	dType string
	dVal  int64
}

type listItem struct {
	val         int64
	idx         int64
	prior, next *listItem
}

func (l *listItem) print() {
	ptr := l
	for ptr != nil {
		fmt.Print(fmt.Sprintf("[(%d) %d],", ptr.idx, ptr.val))
		ptr = ptr.next
	}
	fmt.Println()
}

func (l *listItem) find(idx int64) *listItem {
	ptr := l
	for idx != ptr.idx {
		if idx > ptr.idx {
			ptr = ptr.next
		} else {
			ptr = ptr.prior
		}
		if ptr == nil {
			panic("Traversing a broken list set")
		}
	}
	return ptr
}

func (l *listItem) set(idx, val int64) {
	ptr := l.find(idx)
	ptr.val = val
}

func (l *listItem) get(idx int64) *listItem {
	ptr := l.find(idx)
	return ptr
}

func (l *listItem) slice(start, end int64) *listItem {
	// Find start item
	ptr := l.find(start)

	// Start a new list up to end
	idx := int64(0)
	firstItem := listItem{ptr.val, idx, nil, nil}
	sPtr := &firstItem
	for ptr.idx != end {
		if ptr.next == nil {
			panic("Traversing incomplete list")
		}
		idx++
		ptr = ptr.next

		next := listItem{ptr.val, idx, sPtr, nil}
		sPtr.next = &next
		sPtr = sPtr.next
	}

	// Return pointer to start item
	return &firstItem
}

func (l *listItem) attach(other *listItem) {
	// Wind left
	ptr, ptrO := l, other
	for ptr.prior != nil {
		ptr = ptr.prior
	}
	for ptrO.prior != nil {
		ptrO = ptrO.prior
	}

	// Wind right
	idx := int64(0)
	for {
		ptr.idx = idx
		idx++
		if ptr.next != nil {
			ptr = ptr.next
		} else {
			break
		}
	}

	// Attach & continue
	ptr.next = ptrO
	ptrO.prior = ptr

	for ptr.next != nil {
		ptr = ptr.next
		ptr.idx = idx
		idx++
	}
}

func main() {
	commands := parseInput("./input.txt")

	// ---- P1 ----
	cards := makeDeck(10007)
	shuffled := shuffleDeck(cards, commands, 10007)
	pos := findCard(2019, shuffled)
	fmt.Println("P1", pos)

	// ---- P2 ----
	cards = makeDeck(119315717514047)
	for i := 0; i < 101741582076661; i++ {
		sTime := time.Now()
		cards = shuffleDeck(cards, commands, 119315717514047)
		fmt.Println(time.Since(sTime))
	}
	pos = findCard(2020, cards)
	fmt.Println("P2", pos)
}

func findCard(findMe int64, deck listItem) int64 {
	dkPtr := &deck
	for dkPtr != nil {
		if dkPtr.val == findMe {
			return dkPtr.idx
		}
		dkPtr = dkPtr.next
	}
	return int64(-1)
}

func makeDeck(size int64) listItem {
	start := listItem{0, 0, nil, nil}
	ptr := &start
	for i := int64(1); i < size; i++ {
		next := listItem{i, i, ptr, nil}
		ptr.next = &next
		ptr = ptr.next
	}
	return start
}

func shuffleDeck(deck listItem, commands []command, deckSize int64) listItem {
	for _, cmd := range commands {
		switch cmd.dType {
		case DEAL_INC:
			tmpDeck := makeDeck(deckSize)
			ptr := int64(0)
			dkPtr := &deck
			for dkPtr != nil {
				tmpDeck.set(ptr, dkPtr.val)
				ptr += cmd.dVal
				if ptr >= deckSize {
					ptr -= deckSize
				}
				dkPtr = dkPtr.next
			}
			deck = tmpDeck
		case DEAL_STACK:
			var tmp int64
			for ptr, i := int64(0), deckSize-1; i > ptr; ptr, i = ptr+1, i-1 {
				tmp = deck.get(i).val
				deck.set(i, deck.get(ptr).val)
				deck.set(ptr, tmp)
			}
		case CUT:
			if cmd.dVal < 0 {
				cut := deck.slice(deckSize+cmd.dVal, deckSize-1)
				rest := deck.slice(0, deckSize+cmd.dVal-1)
				cut.attach(rest)
				deck = *(cut.get(0))
			} else {
				cut := deck.slice(0, cmd.dVal-1)
				rest := deck.slice(cmd.dVal, deckSize-1)
				rest.attach(cut)
				deck = *(cut.get(0))
			}
		}
	}
	return deck
}

func parseInput(file string) []command {
	lines := util.GetLines(file)

	var commands []command
	for _, line := range lines {
		var dType string
		var dVal int64

		if line == "deal into new stack" {
			dType = DEAL_STACK
			dVal = 0
		}

		num, err := fmt.Sscanf(line, "deal with increment %d", &dVal)
		if num == 1 && err == nil {
			dType = DEAL_INC
		}

		num, err = fmt.Sscanf(line, "cut %d", &dVal)
		if num == 1 && err == nil {
			dType = CUT
		}

		commands = append(commands, command{dType, dVal})
	}
	return commands
}
