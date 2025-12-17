# Quick Start Guide

## Installation

```bash
npm install protobufjs-rust
```

## Basic Usage

The library works exactly like protobuf.js with automatic performance optimization:

### Example 1: Load and Use a .proto File

```javascript
const protobuf = require("protobufjs-rust");

// Load a .proto file
protobuf.load("awesome.proto", function(err, root) {
    if (err) throw err;

    // Obtain a message type
    const AwesomeMessage = root.lookupType("awesomepackage.AwesomeMessage");

    // Create a message
    const message = AwesomeMessage.create({ 
        awesomeField: "hello" 
    });

    // Encode the message
    const buffer = AwesomeMessage.encode(message).finish();
    console.log("Encoded:", buffer);

    // Decode the message
    const decoded = AwesomeMessage.decode(buffer);
    console.log("Decoded:", decoded);
});
```

### Example 2: Using Static Code

```javascript
const protobuf = require("protobufjs-rust");

// Load pre-compiled static code
const root = protobuf.loadSync("myproto.json");
const MyMessage = root.lookupType("mypackage.MyMessage");

// Create and encode
const message = MyMessage.create({ 
    id: 123, 
    name: "Test" 
});
const buffer = MyMessage.encode(message).finish();

// Decode
const decoded = MyMessage.decode(buffer);
console.log(decoded);
```

### Example 3: Check Performance Mode

```javascript
const protobuf = require("protobufjs-rust");

console.log("Build type:", protobuf.build);
// Output: "rust-wasm-enhanced" or "javascript"

console.log("Rust acceleration:", protobuf.rustAcceleration);
// Output: true or false

// The library automatically uses the best available implementation
// No code changes needed!
```

## Running Benchmarks

Compare performance between different implementations:

```bash
npm run bench
```

Sample output:
```
benchmarking encoding performance ...

protobuf.js (reflect) x 970,808 ops/sec
protobuf.js (static) x 990,800 ops/sec
JSON (string) x 693,263 ops/sec
JSON (buffer) x 516,981 ops/sec
google-protobuf x 512,377 ops/sec

benchmarking decoding performance ...

protobuf.js (reflect) x 2,004,279 ops/sec
protobuf.js (static) x 2,128,607 ops/sec
JSON (string) x 672,613 ops/sec
JSON (buffer) x 587,993 ops/sec
google-protobuf x 469,172 ops/sec
```

## Using CLI Tools

Generate static code from .proto files:

```bash
# Generate JavaScript code
npx pbjs -t static-module -w commonjs -o compiled.js awesome.proto

# Generate TypeScript definitions
npx pbts -o compiled.d.ts compiled.js
```

## Common Use Cases

### Case 1: REST API with Protocol Buffers

```javascript
const express = require('express');
const protobuf = require("protobufjs-rust");

const app = express();

// Load proto
const root = protobuf.loadSync("api.proto");
const Request = root.lookupType("api.Request");
const Response = root.lookupType("api.Response");

app.post('/api', express.raw({ type: 'application/protobuf' }), (req, res) => {
    // Decode request
    const request = Request.decode(req.body);
    
    // Process...
    const response = Response.create({
        status: "success",
        data: processRequest(request)
    });
    
    // Encode response
    const buffer = Response.encode(response).finish();
    res.type('application/protobuf').send(buffer);
});
```

### Case 2: WebSocket with Protocol Buffers

```javascript
const WebSocket = require('ws');
const protobuf = require("protobufjs-rust");

const root = protobuf.loadSync("messages.proto");
const Message = root.lookupType("chat.Message");

const wss = new WebSocket.Server({ port: 8080 });

wss.on('connection', (ws) => {
    ws.on('message', (data) => {
        // Decode incoming message
        const message = Message.decode(data);
        console.log('Received:', message);
        
        // Create response
        const response = Message.create({
            user: "server",
            text: "Message received"
        });
        
        // Encode and send
        ws.send(Message.encode(response).finish());
    });
});
```

### Case 3: File Storage

```javascript
const fs = require('fs');
const protobuf = require("protobufjs-rust");

const root = protobuf.loadSync("data.proto");
const Record = root.lookupType("storage.Record");

// Save to file
function saveRecord(record) {
    const buffer = Record.encode(record).finish();
    fs.writeFileSync('data.bin', buffer);
}

// Load from file
function loadRecord() {
    const buffer = fs.readFileSync('data.bin');
    return Record.decode(buffer);
}

// Usage
saveRecord({ id: 1, name: "Test", timestamp: Date.now() });
const record = loadRecord();
console.log(record);
```

## Debugging

Enable debug logging to see fallback behavior:

```bash
PROTOBUF_DEBUG=1 node your-app.js
```

Output when Rust is not available:
```
protobufjs-rust: Rust WASM not available, using pure JavaScript implementation
To build Rust WASM: npm run build:rust
```

## Performance Tips

1. **Use static code generation** for best performance:
   ```bash
   npx pbjs -t static-module -w commonjs -o compiled.js proto/*.proto
   ```

2. **Reuse message instances** when possible:
   ```javascript
   // Good - reuse
   const message = MyMessage.create();
   for (let data of items) {
       Object.assign(message, data);
       const buffer = MyMessage.encode(message).finish();
       // use buffer...
   }
   
   // Less efficient - recreate each time
   for (let data of items) {
       const message = MyMessage.create(data);
       const buffer = MyMessage.encode(message).finish();
   }
   ```

3. **Enable Rust WASM** for maximum performance:
   ```bash
   npm run build:rust
   ```

## Migration from protobuf.js

If you're already using protobuf.js, migration is simple:

```javascript
// Before
const protobuf = require("protobufjs");

// After - just change the require!
const protobuf = require("protobufjs-rust");

// Everything else stays the same!
```

Your existing code will work without any changes, and you'll automatically get performance benefits when Rust WASM is built.

## Next Steps

- Read the full [README](../README.md) for detailed documentation
- Check [RUST_BUILD.md](RUST_BUILD.md) to enable Rust acceleration
- Review [protobuf.js documentation](https://protobufjs.github.io/protobuf.js/) for API details
- Run `npm run bench` to see performance metrics

## Getting Help

- Check the [protobuf.js documentation](https://protobufjs.github.io/protobuf.js/)
- Review the [examples](../examples/) directory
- Open an issue on GitHub for bugs or questions

---

Happy protobuffing! 🚀
