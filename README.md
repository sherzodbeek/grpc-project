# grpc-project
RPC Protocol (gRPC projects)

1. **gRPC part (40 points)**
 - Create a simple gRPC server application that listens on port 8080 for a request like this (feel free to add more fields) 

```
{      
    "message": "Ping" 
}
```

And responses with something like this (feel free to add more fields)

```
{     
      "message": "Pong" 
}
```

 - Create a simple Java gRPC client application that sends a request to the simple server (share the same proto file), gets a response, and prints it.
 - Create a simple gRPC client application in any other language you like (or ask a trainer/lector for a Python example) that sends a request to the simple server, gets a response, and prints it. Use the same proto file.
 - Start a server and try to communicate with it using the clients you just wrote.
