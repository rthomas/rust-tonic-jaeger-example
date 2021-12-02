default:
  just --list
  
start:
  cargo build
  ./target/debug/maths-add-service &
  sleep 1
  ./target/debug/maths-mul-service &
  sleep 1
  ./target/debug/maths-frontend-service &

stop:
  killall maths-frontend-service | true
  killall maths-mul-service | true
  killall maths-add-service | true

restart: stop start

start-jaeger:
  docker run --rm --name jaeger -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest

stop-jaeger:
  docker stop jaeger