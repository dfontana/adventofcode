package util

import (
	"bufio"
	"io"
	"log"
	"os"
	"regexp"
	"strconv"
)

// OpenFile to read in the given file as an IoReader
func OpenFile(fname string) (io.Reader, error) {
	return os.Open(fname)
}

// GetLines reads in the given file, returning and array of strings - one
// per line in the file.
func GetLines(fileName string) []string {
	file, err := os.Open(fileName)
	if err != nil {
		log.Fatal(err)

	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	lines := []string{}

	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)

	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)

	}

	return lines
}

// Split the given string based in the given regex
func Split(text string, delimeter string) []string {
	reg := regexp.MustCompile(delimeter)
	indexes := reg.FindAllStringIndex(text, -1)
	laststart := 0
	result := make([]string, len(indexes)+1)
	for i, element := range indexes {
		result[i] = text[laststart:element[0]]
		laststart = element[1]

	}
	result[len(indexes)] = text[laststart:len(text)]
	return result
}

// ToInt converts a string to integer
func ToInt(item string) int {
	val, err := strconv.Atoi(item)
	if err != nil {
		log.Fatal(err)
	}
	return val
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
