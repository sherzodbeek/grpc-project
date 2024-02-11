package main

import (
	"context"
	"fmt"
	"learn/client/message"
	"log"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	opts := grpc.WithTransportCredentials(insecure.NewCredentials())
	cc, err := grpc.Dial("localhost:50051", opts)
	if err != nil {
		log.Fatal(err)
	}
	defer cc.Close()

	client := message.NewHelloClient(cc)
	request := &message.HelloMessageRequest{Message: "Hello from GO"}

	resp, err := client.SayHello(context.Background(), request)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Receive response => %s ", resp.Message)
}
