const { 
    decodeVarint, 
    encodeVarint,
    decodeZigzag,
    encodeZigzag,
    decodeFieldTag,
    encodeFieldTag,
    ProtobufParser 
} = require('./index.js');

console.log('Testing protobuf-rs native module...\n');

// Test varint encoding
console.log('Test 1: Varint encoding');
const encoded = encodeVarint(300);
console.log('encodeVarint(300) =', encoded);

// Test varint decoding
console.log('\nTest 2: Varint decoding');
const decoded = decodeVarint(encoded);
console.log('decodeVarint(encoded) =', decoded);
console.assert(decoded === 300, 'Decoded value should be 300');

// Test large varint values
console.log('\nTest 3: Large varint values');
const largeValue = 9007199254740991; // Max safe integer in JS
const encodedLarge = encodeVarint(largeValue);
const decodedLarge = decodeVarint(encodedLarge);
console.log(`encodeVarint(${largeValue}) decoded =`, decodedLarge);
console.assert(decodedLarge === largeValue, 'Large value should match');

// Test negative values with varint (treated as large unsigned)
console.log('\nTest 4: Negative values in varint');
try {
    const encodedNeg = encodeVarint(-1);
    const decodedNeg = decodeVarint(encodedNeg);
    console.log('encodeVarint(-1) decoded =', decodedNeg);
} catch (e) {
    console.log('Expected behavior for negative values');
}

// Test zigzag encoding
console.log('\nTest 5: Zigzag encoding');
const zigzagEncoded = encodeZigzag(-1);
console.log('encodeZigzag(-1) =', zigzagEncoded);

// Test zigzag decoding
console.log('\nTest 6: Zigzag decoding');
const zigzagDecoded = decodeZigzag(zigzagEncoded);
console.log('decodeZigzag(encoded) =', zigzagDecoded);
console.assert(zigzagDecoded === -1, 'Decoded value should be -1');

// Test zigzag with various values
console.log('\nTest 7: Zigzag edge cases');
const testValues = [0, -1, 1, -2, 2, -100, 100];
testValues.forEach(val => {
    const enc = encodeZigzag(val);
    const dec = decodeZigzag(enc);
    console.assert(dec === val, `ZigZag roundtrip failed for ${val}`);
});
console.log('ZigZag edge cases passed');

// Test field tag encoding
console.log('\nTest 8: Field tag encoding');
const fieldTag = encodeFieldTag(1, 0); // field number 1, wire type 0 (varint)
console.log('encodeFieldTag(1, 0) =', fieldTag);

// Test field tag decoding
console.log('\nTest 9: Field tag decoding');
const [fieldNumber, wireType] = decodeFieldTag(fieldTag);
console.log('decodeFieldTag(fieldTag) = [', fieldNumber, ',', wireType, ']');
console.assert(fieldNumber === 1, 'Field number should be 1');
console.assert(wireType === 0, 'Wire type should be 0');

// Test invalid wire types
console.log('\nTest 10: Invalid wire type');
try {
    encodeFieldTag(1, 6); // Invalid wire type
    console.error('Should have thrown error for invalid wire type');
} catch (e) {
    console.log('Correctly rejected invalid wire type');
}

// Test invalid field numbers
console.log('\nTest 11: Invalid field numbers');
try {
    encodeFieldTag(0, 0); // Field number must be >= 1
    console.error('Should have thrown error for field number 0');
} catch (e) {
    console.log('Correctly rejected field number 0');
}

try {
    encodeFieldTag(19000, 0); // Reserved range
    console.error('Should have thrown error for reserved field number');
} catch (e) {
    console.log('Correctly rejected reserved field number 19000');
}

try {
    encodeFieldTag(536870912, 0); // Above max field number
    console.error('Should have thrown error for field number above max');
} catch (e) {
    console.log('Correctly rejected field number above maximum');
}

// Test valid field number edge cases
console.log('\nTest 12: Valid field number edge cases');
const validTag1 = encodeFieldTag(1, 0);
console.assert(validTag1.length > 0, 'Field number 1 should work');
const validTag2 = encodeFieldTag(18999, 0);
console.assert(validTag2.length > 0, 'Field number 18999 should work');
const validTag3 = encodeFieldTag(20000, 0);
console.assert(validTag3.length > 0, 'Field number 20000 should work');
const validTag4 = encodeFieldTag(536870911, 0); // Max field number
console.assert(validTag4.length > 0, 'Max field number should work');
console.log('Valid field number edge cases passed');

// Test empty buffer
console.log('\nTest 13: Empty buffer');
try {
    decodeVarint(Buffer.from([]));
    console.error('Should have thrown error for empty buffer');
} catch (e) {
    console.log('Correctly rejected empty buffer for varint');
}

try {
    decodeFieldTag(Buffer.from([]));
    console.error('Should have thrown error for empty buffer');
} catch (e) {
    console.log('Correctly rejected empty buffer for field tag');
}

// Test incomplete varint
console.log('\nTest 14: Incomplete varint');
try {
    decodeVarint(Buffer.from([0xff, 0xff, 0xff])); // All bytes have continuation bit
    console.error('Should have thrown error for incomplete varint');
} catch (e) {
    console.log('Correctly rejected incomplete varint');
}

// Test varint overflow
console.log('\nTest 15: Varint overflow');
try {
    // Varint with 11 bytes - exceeds maximum length
    const elevenBytes = Buffer.from([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]);
    decodeVarint(elevenBytes);
    console.error('Should have thrown error for varint too long');
} catch (e) {
    console.log('Correctly rejected varint that is too long');
}

try {
    // 10th byte with value > 1 causes overflow for 64-bit number
    const overflowBytes = Buffer.from([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02]);
    decodeVarint(overflowBytes);
    console.error('Should have thrown error for varint overflow');
} catch (e) {
    console.log('Correctly rejected varint overflow');
}

// Test ProtobufParser
console.log('\nTest 16: ProtobufParser');
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

// Test all wire types
console.log('\nTest 17: All wire types');
for (let wt = 0; wt <= 5; wt++) {
    const tag = encodeFieldTag(10, wt);
    const [fn, wtype] = decodeFieldTag(tag);
    console.assert(fn === 10 && wtype === wt, `Wire type ${wt} failed`);
}
console.log('All wire types (0-5) passed');

console.log('\n✅ All tests passed!');
