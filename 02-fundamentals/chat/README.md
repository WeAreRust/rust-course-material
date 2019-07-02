# Chat workshop

## Running

### Server

```sh
chat server -p $PORT
```

### Client

```sh
chat client -s $SERVER_IP -m "$CHANNEL|$NAME|$MESSAGE"
```

### Clock example

```sh
while true; do TIME=$(date "+%H:%M:%S"); chat client -s "127.0.0.1:9999" -m "time|clock|the time is: $TIME"; sleep 1; done
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
