package intcode

type config struct {
	Input   chan int64
	Output  chan int64
	Done    chan bool
	Request chan bool

	sendDoneSignal    bool
	sendRequestSignal bool
}

func (c config) SetInput(in chan int64) config {
	c.Input = in
	return c
}

func (c config) SetOutput(out chan int64) config {
	c.Output = out
	return c
}

func (c config) SetDone(done chan bool) config {
	c.Done = done
	return c
}

func (c config) SendDone() config {
	c.sendDoneSignal = true
	return c
}

func (c config) SendRequest() config {
	c.sendRequestSignal = true
	return c
}

// Config for new run
func Config() config {
	return config{
		Input:             make(chan int64),
		Output:            make(chan int64),
		Done:              make(chan bool, 1),
		Request:           make(chan bool),
		sendDoneSignal:    false,
		sendRequestSignal: false,
	}
}
