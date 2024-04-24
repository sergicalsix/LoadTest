package main

import (
    "encoding/csv"
    "net/http"
    "os"
    "time"
)

func root(w http.ResponseWriter, r *http.Request) {
    w.Header().Set("Content-Type", "application/json")
    w.Write([]byte(`{"message": "Hello, World!"}`))
}

func fileRead(w http.ResponseWriter, r *http.Request) {
    file, err := os.Open("../data/tested.csv")
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
        return
    }
    defer file.Close()
    csvReader := csv.NewReader(file)
    for {
        _, err := csvReader.Read()
        if err != nil {
            break
        }
    }
    w.Header().Set("Content-Type", "application/json")
    w.Write([]byte(`{"message": "file_read"}`))
}


func waitTime(w http.ResponseWriter, r *http.Request) {
    time.Sleep(1 * time.Second)
    w.Header().Set("Content-Type", "application/json")
    w.Write([]byte(`{"message": "time_sleep"}`))
}


func main() {
    http.HandleFunc("/", root)
    http.HandleFunc("/file_read", fileRead)
    http.HandleFunc("/wait_time", waitTime)
    http.ListenAndServe(":8000", nil)
}