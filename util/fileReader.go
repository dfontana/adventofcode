package util

import (
	"bufio"
	"io"
	"os"
	"strconv"
)

// OpenFile to read in the given file as an IoReader
func OpenFile(fname string) (io.Reader, error) {
	return os.Open(fname)
}

// ReadInts reads whitespace-separated ints from r. If there's an error, it
// returns the ints successfully read so far as well as the error value.
func ReadInts(r io.Reader) ([]int, error) {
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanWords)
	var result []int
	for scanner.Scan() {
		x, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return result, err

		}
		result = append(result, x)

	}
	return result, scanner.Err()

}
