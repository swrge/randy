FROM golang:bookworm as builder

# Builder Stage
WORKDIR /app

COPY go.* ./
RUN go mod download

COPY . ./

RUN go build -mod=readonly -o server

# Running Stage
FROM gcr.io/distroless/base-debian12

COPY --from=builder /app/server /app/server

ENV PORT 8080
EXPOSE 8080

CMD ["/app/server"]
