package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strconv"
	"strings"
)

// Response is a struct used to parse json requests and responses.
type Response struct {
	Value string `json:"value"`
}

func handleAnalogInput(w http.ResponseWriter, r *http.Request, rpc *RPC) {
	id, _ := strconv.Atoi(strings.Split(r.URL.Path, "/")[4])
	val := rpc.readAnalog(id)
	json, _ := json.Marshal(Response{strconv.FormatFloat(val, 'f', -1, 64)})
	w.Header().Set("Content-Type", "application/json")
	w.Write(json)
}

func handleAnalogOutput(w http.ResponseWriter, r *http.Request, rpc *RPC) {
	var args Response

	id, _ := strconv.Atoi(strings.Split(r.URL.Path, "/")[4])

	decoder := json.NewDecoder(r.Body)
	_ = decoder.Decode(&args)
	output, _ := strconv.Atoi(args.Value)
	val := rpc.writeAnalog(id, output)

	json, _ := json.Marshal(Response{strconv.FormatFloat(val, 'f', -1, 64)})
	w.Header().Set("Content-Type", "application/json")
	w.Write(json)
}

func handleDigitalInput(w http.ResponseWriter, r *http.Request, rpc *RPC) {
	id, _ := strconv.Atoi(strings.Split(r.URL.Path, "/")[4])
	val := rpc.readDigital(id)
	json, _ := json.Marshal(Response{strconv.FormatFloat(val, 'f', -1, 64)})
	w.Header().Set("Content-Type", "application/json")
	w.Write(json)
}

func handleDigitalOutput(w http.ResponseWriter, r *http.Request, rpc *RPC) {
	var args Response

	id, _ := strconv.Atoi(strings.Split(r.URL.Path, "/")[4])

	decoder := json.NewDecoder(r.Body)
	_ = decoder.Decode(&args)
	output, _ := strconv.Atoi(args.Value)
	val := rpc.writeDigital(id, output)

	json, _ := json.Marshal(Response{strconv.FormatFloat(val, 'f', -1, 64)})
	w.Header().Set("Content-Type", "application/json")
	w.Write(json)
}

func main() {
	http.HandleFunc("/api/analog/input/", func(w http.ResponseWriter, r *http.Request) {
		rpc := NewRPC()
		defer rpc.Close()
		handleAnalogInput(w, r, rpc)
	})
	http.HandleFunc("/api/analog/output/", func(w http.ResponseWriter, r *http.Request) {
		rpc := NewRPC()
		defer rpc.Close()
		handleAnalogOutput(w, r, rpc)
	})

	http.HandleFunc("/api/digital/input/", func(w http.ResponseWriter, r *http.Request) {
		rpc := NewRPC()
		defer rpc.Close()
		handleDigitalInput(w, r, rpc)
	})

	http.HandleFunc("/api/digital/output/", func(w http.ResponseWriter, r *http.Request) {
		rpc := NewRPC()
		defer rpc.Close()
		handleDigitalOutput(w, r, rpc)
	})

    fmt.Println("Serving on 1337")
	http.ListenAndServe(":1337", nil)

	fmt.Println("the end")
}
