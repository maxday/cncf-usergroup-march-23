FROM golang:1.20 as builder
WORKDIR /app
COPY pizza_order.go provider.go go.mod go.sum /app/
RUN go build -o pizza_order

FROM gcr.io/distroless/base-debian11
COPY --from=builder /app/pizza_order /app/pizza_order
CMD ["/app/pizza_order"]