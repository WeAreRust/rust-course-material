# Chat workshop

## Running

### Server

```sh
cargo run -- server -p $PORT
```

### Client

```sh
cargo run -- client -s $SERVER_IP -m "$CHANNEL|$NAME|$MESSAGE"
```

### Clock example

```sh
while true; do TIME=$(date "+%H:%M:%S"); cargo run -- client -s "127.0.0.1:9999" -m "time|clock|the time is: $TIME"; sleep 1; done
```

## Protocol

### Subscribe
```
S|$CHANNEL
```

### Unsubscribe
```
U|$CHANNEL
```

### Publish
```
P|$CHANNEL|$NAME|$MESSAGE
```
