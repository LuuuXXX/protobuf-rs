# protobuf-rs

A Rust native module for protobuf.js using NAPI-RS, providing high-performance Protocol Buffer operations.

## Features

- 🚀 High-performance Protocol Buffer operations powered by Rust
- 🔧 Varint encoding and decoding
- 🔄 ZigZag encoding and decoding for signed integers
- 🏷️ Field tag encoding and decoding
- 📦 Protobuf message parsing
- 🌐 Cross-platform support via NAPI-RS
- 💪 Type-safe TypeScript bindings

## Installation

```bash
npm install protobuf-rs
```

## Usage

```javascript
const { 
    decodeVarint, 
    encodeVarint, 
    decodeZigzag,
    encodeZigzag,
    decodeFieldTag,
    encodeFieldTag,
    ProtobufParser 
} = require('protobuf-rs');

// Encode a varint
const encoded = encodeVarint(300);
console.log(encoded); // <Buffer ac 02>

// Decode a varint
const decoded = decodeVarint(encoded);
console.log(decoded); // 300

// ZigZag encoding for signed integers
const zigzagEncoded = encodeZigzag(-1);
console.log(zigzagEncoded); // 1

const zigzagDecoded = decodeZigzag(1);
console.log(zigzagDecoded); // -1

// Field tag operations
const tag = encodeFieldTag(1, 0); // field number 1, wire type 0
const [fieldNumber, wireType] = decodeFieldTag(tag);
console.log(fieldNumber, wireType); // 1, 0

// Parse protobuf data
const parser = new ProtobufParser();
const buffer = Buffer.from([0x08, 0x96, 0x01]);
parser.parse(buffer);
console.log(parser.getSize()); // 3
console.log(parser.getData()); // <Buffer 08 96 01>
```

## Building from Source

```bash
# Install dependencies
npm install

# Build the native module
npm run build

# Run tests
npm test
```

## Development

```bash
# Build in debug mode
npm run build:debug

# Build for release
npm run build
```

## API

### Varint Operations

#### `encodeVarint(value: number): Buffer`

Encodes a 64-bit signed integer as a Protocol Buffer varint.

**Parameters:**
- `value` - The integer to encode

**Returns:** A Buffer containing the encoded varint

#### `decodeVarint(buffer: Buffer): number`

Decodes a Protocol Buffer varint from a buffer.

**Parameters:**
- `buffer` - The buffer containing the varint

**Returns:** The decoded integer value

### ZigZag Operations

#### `encodeZigzag(value: number): number`

Encodes a signed integer using ZigZag encoding. This is useful for encoding signed integers
efficiently, as it maps signed integers to unsigned integers in a way that small absolute
values have small encoded values.

**Parameters:**
- `value` - The signed integer to encode

**Returns:** The ZigZag encoded value

#### `decodeZigzag(value: number): number`

Decodes a ZigZag encoded integer back to a signed integer.

**Parameters:**
- `value` - The ZigZag encoded value

**Returns:** The decoded signed integer

### Field Tag Operations

#### `encodeFieldTag(fieldNumber: number, wireType: number): Buffer`

Encodes a Protocol Buffer field tag.

**Parameters:**
- `fieldNumber` - The field number (must be >= 0)
- `wireType` - The wire type (0-5)

**Returns:** A Buffer containing the encoded tag

**Wire Types:**
- 0: Varint
- 1: 64-bit
- 2: Length-delimited
- 3: Start group (deprecated)
- 4: End group (deprecated)
- 5: 32-bit

#### `decodeFieldTag(buffer: Buffer): Array<number>`

Decodes a Protocol Buffer field tag.

**Parameters:**
- `buffer` - The buffer containing the field tag

**Returns:** An array `[fieldNumber, wireType]`

### ProtobufParser

A class for parsing Protocol Buffer messages.

#### Methods

##### `constructor()`

Creates a new parser instance.

##### `parse(buffer: Buffer): string`

Parses a buffer and stores the data internally.

**Parameters:**
- `buffer` - The buffer to parse

**Returns:** A status message indicating the number of bytes parsed

##### `getSize(): number`

Returns the size of the last parsed buffer.

**Returns:** The buffer size in bytes

##### `getData(): Buffer`

Returns a copy of the last parsed buffer data.

**Returns:** A Buffer containing the parsed data

## Performance

This module is implemented in Rust and uses NAPI-RS for seamless Node.js integration,
providing significant performance improvements over pure JavaScript implementations,
especially for large datasets.

## License

MIT
