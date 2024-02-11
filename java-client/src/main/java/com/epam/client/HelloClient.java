package com.epam.client;

import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import message.HelloGrpc;
import message.Message;

import java.util.logging.Logger;

public class HelloClient {

    private static final Logger logger = Logger.getLogger(HelloClient.class.getName());

    public static void main(String[] args) {
        logger.info("JavaClient is calling Rust Server method");
        ManagedChannel managedChannel = ManagedChannelBuilder.forTarget("localhost:50051")
                .usePlaintext().build();

        HelloGrpc.HelloBlockingStub helloBlockingStub = HelloGrpc.newBlockingStub(managedChannel);
        
        Message.HelloMessageRequest request = Message.HelloMessageRequest.newBuilder()
                .setMessage("Hello from Java")
                .build();
        Message.HelloMessageReponse response = helloBlockingStub.sayHello(request);
        System.out.println(response.getMessage());
    }

}
