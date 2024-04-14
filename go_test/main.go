package main

import (
    "encoding/json"
    "log"
    "net/http"
)

func main() {
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        response := map[string]string{"message": "Hello, World!"}
        jsonResponse, err := json.Marshal(response)
        if err != nil {
            http.Error(w, err.Error(), http.StatusInternalServerError)
            return
        }

        w.Header().Set("Content-Type", "application/json")
        w.Write(jsonResponse)
    })

    log.Fatal(http.ListenAndServe(":8000", nil))
}