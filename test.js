const { 
    sum, 
    decodeVarint, 
    encodeVarint,
    decodeZigzag,
    encodeZigzag,
    decodeFieldTag,
    encodeFieldTag,
    ProtobufParser 
} = require('./index.js');

console.log('Testing protobuf-rs native module...\n');

// Test simple sum function
console.log('Test 1: sum(2, 3) =', sum(2, 3));
console.assert(sum(2, 3) === 5, 'sum should return 5');

// Test varint encoding
console.log('\nTest 2: Varint encoding');
const encoded = encodeVarint(300);
console.log('encodeVarint(300) =', encoded);

// Test varint decoding
console.log('\nTest 3: Varint decoding');
const decoded = decodeVarint(encoded);
console.log('decodeVarint(encoded) =', decoded);
console.assert(decoded === 300, 'Decoded value should be 300');

// Test zigzag encoding
console.log('\nTest 4: Zigzag encoding');
const zigzagEncoded = encodeZigzag(-1);
console.log('encodeZigzag(-1) =', zigzagEncoded);

// Test zigzag decoding
console.log('\nTest 5: Zigzag decoding');
const zigzagDecoded = decodeZigzag(zigzagEncoded);
console.log('decodeZigzag(encoded) =', zigzagDecoded);
console.assert(zigzagDecoded === -1, 'Decoded value should be -1');

// Test field tag encoding
console.log('\nTest 6: Field tag encoding');
const fieldTag = encodeFieldTag(1, 0); // field number 1, wire type 0 (varint)
console.log('encodeFieldTag(1, 0) =', fieldTag);

// Test field tag decoding
console.log('\nTest 7: Field tag decoding');
const [fieldNumber, wireType] = decodeFieldTag(fieldTag);
console.log('decodeFieldTag(fieldTag) = [', fieldNumber, ',', wireType, ']');
console.assert(fieldNumber === 1, 'Field number should be 1');
console.assert(wireType === 0, 'Wire type should be 0');

// Test ProtobufParser
console.log('\nTest 8: ProtobufParser');
const parser = new ProtobufParser();
const buffer = Buffer.from([0x08, 0x96, 0x01]);
const result = parser.parse(buffer);
console.log('parse result:', result);
console.log('buffer size:', parser.getSize());
console.assert(parser.getSize() === 3, 'Buffer size should be 3');

// Test getData
const data = parser.getData();
console.log('getData():', data);
console.assert(data.length === 3, 'Data length should be 3');

console.log('\n✅ All tests passed!');
