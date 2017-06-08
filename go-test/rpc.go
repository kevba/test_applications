package main

import (
	"fmt"

	"github.com/powerman/rpc-codec/jsonrpc2"
)

const (
	//IrisSocket is the path to the iris socket.
	IrisSocket = "/tmp/iris"
)

// RPC is a struct for calling iris jsonrpc api
type RPC struct {
	Client *jsonrpc2.Client
}

// NewRPC is a function to Create a new RPC struct
func NewRPC() *RPC {
	c, _ := jsonrpc2.Dial("unix", IrisSocket)
	return &RPC{Client: c}
}

// Close closes the rpc connection
func (r *RPC) Close() {
	r.Client.Close()
}

func (r *RPC) readAnalog(id int) float64 {
	var reply float64
	args := make([]interface{}, 2)

	args[0] = id
	args[1] = false

	err := r.Client.Call("read_analog", args, &reply)
	if err != nil {
		fmt.Println(err)
	}
	return reply
}

func (r *RPC) readDigital(id int) float64 {
	var reply float64
	args := make([]interface{}, 1)

	args[0] = id

	err := r.Client.Call("read_digital", args, &reply)
	if err != nil {
		fmt.Println(err)
	}

	return reply
}

func (r *RPC) writeAnalog(id int, val int) float64 {
	var reply float64
	args := make([]interface{}, 3)

	args[0] = id
	args[1] = val
	args[2] = false

	err := r.Client.Call("write_analog", args, &reply)
	if err != nil {
		fmt.Println(err)
	}
	return reply
}

func (r *RPC) writeDigital(id int, val int) float64 {
	var reply float64
	args := make([]interface{}, 2)

	args[0] = id
	args[1] = val

	err := r.Client.Call("write_digital", args, &reply)
	if err != nil {
		fmt.Println(err)
	}
	return reply
}
