package main

//go:generate go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
//go:generate go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
//go:generate protoc --go_out=./ --go-grpc_out=./ --proto_path=../proto ../proto/greeter.proto

import (
	"context"
	"flag"
	"fmt"
	"log"
	"net"
	"net/http"
	"sync"

	pb "github.com/FrancescoLuzzi/test_proto/go_proto/greeter"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

var (
	grpc_port = flag.Int("grpc_port", 50051, "The grpc server port")
	http_port = flag.Int("http_port", 80, "The http server port")
	grpc_dst  = flag.String("grpc_dst", "http://localhost:50051", "Destination grpc server to call")
)

// server is used to implement helloworld.GreeterServer.
type server struct {
	pb.UnimplementedGreeterServer
}

// SayHello implements helloworld.GreeterServer
func (s *server) Greet(ctx context.Context, in *pb.GreetRequest) (*pb.GreetReply, error) {
	log.Printf("Received: %v", in.GetName())
	return &pb.GreetReply{Answer: "Hello " + in.GetName()}, nil
}

func rootHandler(w http.ResponseWriter, r *http.Request) {
	conn, err := grpc.NewClient(*grpc_dst, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		http.Error(w, fmt.Sprintf("Something went wrong while connecting: %v", err), 500)
		return
	}
	client := pb.NewGreeterClient(conn)
	ctx := context.Background()
	ret, err := client.Greet(ctx, &pb.GreetRequest{Name: "Hello from Golang"})
	if err != nil {
		http.Error(w, fmt.Sprintf("Error when requesting: %v", err), 500)
		return
	}
	fmt.Fprintf(w, "%v", ret.Answer)
}

func main() {
	flag.Parse()
	log.SetFlags(log.Ldate | log.Ltime)
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", *grpc_port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()
	pb.RegisterGreeterServer(s, &server{})
	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		log.Printf("grpc server listening at %v", lis.Addr())
		defer wg.Done()
		if err := s.Serve(lis); err != nil {
			log.Fatalf("failed to serve grpc: %v", err)
		}
	}()

	go func() {
		defer wg.Done()
		http.HandleFunc("/", rootHandler)
		log.Printf("http server listening at :%d", *http_port)
		if err := http.ListenAndServe(fmt.Sprintf(":%d", *http_port), nil); err != nil {
			log.Fatalf("failed to serve http: %v", err)
		}
	}()
	wg.Wait()

}
