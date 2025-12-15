/// <reference types="node" />

/**
 * @protobuf-rs/core - High-performance Protocol Buffers for Node.js
 * 
 * TypeScript definitions for the protobuf-rs native module.
 * This module provides high-performance Protocol Buffer encoding/decoding
 * powered by Rust and NAPI-RS.
 * 
 * @module @protobuf-rs/core
 * @version 1.0.0
 * @license BSD-3-Clause
 */

// =============================================================================
// Core Encoding/Decoding Functions
// =============================================================================

/**
 * Encode a 64-bit integer as a Protocol Buffer varint
 * 
 * Varint encoding uses variable-length encoding where smaller values
 * use fewer bytes (1-10 bytes for 64-bit integers).
 * 
 * @param value - 64-bit signed integer to encode
 * @returns Encoded byte buffer (1-10 bytes)
 * 
 * @example
 * ```typescript
 * const encoded = encodeVarint(300);
 * // Returns Buffer: [0xAC, 0x02]
 * ```
 */
export function encodeVarint(value: number): Buffer;

/**
 * Decode a Protocol Buffer varint
 * 
 * @param buffer - Buffer containing the varint
 * @returns Decoded 64-bit signed integer
 * @throws {Error} If varint is too long, overflows, or is incomplete
 * 
 * @example
 * ```typescript
 * const value = decodeVarint(Buffer.from([0xAC, 0x02]));
 * // Returns: 300
 * ```
 */
export function decodeVarint(buffer: Buffer): number;

/**
 * Encode a signed integer using ZigZag encoding
 * 
 * ZigZag encoding maps signed integers to unsigned integers so that
 * small absolute values correspond to small encoded values.
 * Used for sint32 and sint64 types in Protocol Buffers.
 * 
 * @param value - Signed integer to encode
 * @returns ZigZag encoded value (as unsigned)
 * 
 * @example
 * ```typescript
 * encodeZigzag(0);   // Returns: 0
 * encodeZigzag(-1);  // Returns: 1
 * encodeZigzag(1);   // Returns: 2
 * encodeZigzag(-2);  // Returns: 3
 * ```
 */
export function encodeZigzag(value: number): number;

/**
 * Decode a ZigZag encoded integer
 * 
 * @param value - ZigZag encoded value
 * @returns Decoded signed integer
 * 
 * @example
 * ```typescript
 * decodeZigzag(0);  // Returns: 0
 * decodeZigzag(1);  // Returns: -1
 * decodeZigzag(2);  // Returns: 1
 * ```
 */
export function decodeZigzag(value: number): number;

/**
 * Encode a Protocol Buffer field tag
 * 
 * A field tag combines the field number and wire type into a single varint.
 * Formula: tag = (field_number << 3) | wire_type
 * 
 * @param fieldNumber - Field number (1 to 536,870,911, excluding reserved range 19,000-19,999)
 * @param wireType - Wire type (0-5)
 *   - 0: Varint (int32, int64, uint32, uint64, sint32, sint64, bool, enum)
 *   - 1: 64-bit (fixed64, sfixed64, double)
 *   - 2: Length-delimited (string, bytes, embedded messages, packed repeated)
 *   - 3: Start group (deprecated)
 *   - 4: End group (deprecated)
 *   - 5: 32-bit (fixed32, sfixed32, float)
 * @returns Encoded field tag buffer
 * @throws {Error} If field number or wire type is invalid
 * 
 * @example
 * ```typescript
 * const tag = encodeFieldTag(1, 0);  // Field 1, wire type 0 (varint)
 * ```
 */
export function encodeFieldTag(fieldNumber: number, wireType: number): Buffer;

/**
 * Decode a Protocol Buffer field tag
 * 
 * @param buffer - Buffer containing the field tag
 * @returns Array containing [field_number, wire_type]
 * @throws {Error} If buffer is empty or tag is invalid
 * 
 * @example
 * ```typescript
 * const [fieldNumber, wireType] = decodeFieldTag(buffer);
 * ```
 */
export function decodeFieldTag(buffer: Buffer): number[];

// =============================================================================
// ProtobufParser Class
// =============================================================================

/**
 * Protocol Buffer message parser
 * 
 * Used for parsing and storing Protocol Buffer message data.
 */
export class ProtobufParser {
  /**
   * Create a new parser instance
   */
  constructor();

  /**
   * Parse buffer and store data
   * 
   * @param buffer - The buffer to parse
   * @returns Parse status message
   */
  parse(buffer: Buffer): string;

  /**
   * Get the size of parsed data in bytes
   * 
   * @returns Size in bytes
   */
  getSize(): number;

  /**
   * Get a copy of the parsed data
   * 
   * @returns Copy of the parsed data
   */
  getData(): Buffer;
}

// =============================================================================
// Reader Class
// =============================================================================

/**
 * Protocol Buffer zero-copy reader
 * 
 * Provides high-performance reading of protobuf wire format data
 * with zero-copy optimizations where possible.
 */
export class Reader {
  /**
   * Create a new Reader from a buffer
   * 
   * @param buffer - The buffer to read from
   */
  constructor(buffer: Buffer);

  /**
   * Get current position in the buffer
   * 
   * @returns Current position
   */
  pos(): number;

  /**
   * Get total length of the buffer
   * 
   * @returns Buffer length
   */
  len(): number;

  /**
   * Check if at end of buffer
   * 
   * @returns true if at end of buffer
   */
  isEmpty(): boolean;

  /**
   * Read a varint as uint32
   * 
   * @returns Decoded 32-bit unsigned integer
   * @throws {Error} If buffer underflow, varint overflow, or varint too long
   */
  uint32(): number;

  /**
   * Read a varint as int32
   * 
   * @returns Decoded 32-bit signed integer
   */
  int32(): number;

  /**
   * Read a varint as sint32 (zigzag encoded)
   * 
   * @returns Decoded 32-bit signed integer
   */
  sint32(): number;

  /**
   * Read a fixed32 value (little-endian)
   * 
   * @returns 32-bit unsigned integer
   */
  fixed32(): number;

  /**
   * Read a sfixed32 value (little-endian)
   * 
   * @returns 32-bit signed integer
   */
  sfixed32(): number;

  /**
   * Read a varint as uint64
   * 
   * Note: JavaScript numbers are 64-bit floats and can only safely represent
   * integers up to 2^53-1. Values larger than this may lose precision.
   * 
   * @returns Decoded 64-bit unsigned integer (as number)
   */
  uint64(): number;

  /**
   * Read a varint as int64
   * 
   * Note: JavaScript numbers are 64-bit floats and can only safely represent
   * integers up to 2^53-1. Values larger than this may lose precision.
   * 
   * @returns Decoded 64-bit signed integer (as number)
   */
  int64(): number;

  /**
   * Read a varint as sint64 (zigzag encoded)
   * 
   * Note: JavaScript numbers are 64-bit floats and can only safely represent
   * integers up to 2^53-1. Values larger than this may lose precision.
   * 
   * @returns 64-bit signed integer (as number)
   */
  sint64(): number;

  /**
   * Read a fixed64 value (little-endian)
   * 
   * Note: JavaScript numbers are 64-bit floats and can only safely represent
   * integers up to 2^53-1. Values larger than this may lose precision.
   * 
   * @returns 64-bit unsigned integer (as number)
   */
  fixed64(): number;

  /**
   * Read a sfixed64 value (little-endian)
   * 
   * Note: JavaScript numbers are 64-bit floats and can only safely represent
   * integers up to 2^53-1. Values larger than this may lose precision.
   * 
   * @returns 64-bit signed integer (as number)
   */
  sfixed64(): number;

  /**
   * Read a float value (32-bit IEEE 754)
   * 
   * @returns Float value
   */
  float(): number;

  /**
   * Read a double value (64-bit IEEE 754)
   * 
   * @returns Double value
   */
  double(): number;

  /**
   * Read a bool value
   * 
   * @returns Boolean value
   */
  bool(): boolean;

  /**
   * Read a string with length prefix (zero-copy when possible)
   * 
   * Format: [length: varint][utf8_data: bytes]
   * 
   * @returns Decoded string
   * @throws {Error} If buffer underflow or invalid UTF-8
   */
  string(): string;

  /**
   * Read bytes with length prefix (zero-copy when possible)
   * 
   * Format: [length: varint][data: bytes]
   * 
   * @returns Byte array
   * @throws {Error} If buffer underflow
   */
  bytes(): Buffer;

  /**
   * Skip specified number of bytes
   * 
   * Useful for skipping unknown fields or unwanted data.
   * 
   * @param n - Number of bytes to skip
   * @returns this (for chaining)
   * @throws {Error} If buffer underflow
   */
  skip(n: number): this;

  /**
   * Skip a field by wire type
   * 
   * Automatically skips the correct number of bytes based on wire type.
   * 
   * @param wireType - Wire type (0-5)
   * @returns this (for chaining)
   * @throws {Error} If invalid wire type or buffer underflow
   */
  skipType(wireType: number): this;

  /**
   * Reset position to start
   * 
   * Allows reusing the reader without recreating it.
   */
  reset(): void;
}

// =============================================================================
// Writer Class
// =============================================================================

/**
 * Protocol Buffer buffered writer
 * 
 * Provides high-performance writing of protobuf wire format data
 * with buffer optimization and reuse.
 */
export class Writer {
  /**
   * Create a new Writer
   */
  constructor();

  /**
   * Create a Writer with pre-allocated capacity
   * 
   * Pre-allocating capacity can reduce memory reallocations for better performance.
   * 
   * @param capacity - Initial capacity in bytes
   * @returns New Writer instance
   * 
   * @example
   * ```typescript
 * const writer = Writer.withCapacity(1024);  // Pre-allocate 1KB
   * ```
   */
  static withCapacity(capacity: number): Writer;

  /**
   * Write a uint32 value as varint
   * 
   * @param value - 32-bit unsigned integer
   * @returns this (for chaining)
   */
  uint32(value: number): this;

  /**
   * Write an int32 value as varint
   * 
   * @param value - 32-bit signed integer
   * @returns this (for chaining)
   */
  int32(value: number): this;

  /**
   * Write a sint32 value (zigzag encoded)
   * 
   * @param value - 32-bit signed integer
   * @returns this (for chaining)
   */
  sint32(value: number): this;

  /**
   * Write a fixed32 value (little-endian)
   * 
   * @param value - 32-bit unsigned integer
   * @returns this (for chaining)
   */
  fixed32(value: number): this;

  /**
   * Write a sfixed32 value (little-endian)
   * 
   * @param value - 32-bit signed integer
   * @returns this (for chaining)
   */
  sfixed32(value: number): this;

  /**
   * Write a uint64 value as varint
   * 
   * Note: JavaScript numbers are 64-bit floats and can only safely represent
   * integers up to 2^53-1. Values larger than this may lose precision.
   * 
   * @param value - 64-bit unsigned integer (as number)
   * @returns this (for chaining)
   */
  uint64(value: number): this;

  /**
   * Write an int64 value as varint
   * 
   * Note: JavaScript numbers are 64-bit floats and can only safely represent
   * integers up to 2^53-1. Values larger than this may lose precision.
   * 
   * @param value - 64-bit signed integer (as number)
   * @returns this (for chaining)
   */
  int64(value: number): this;

  /**
   * Write a sint64 value (zigzag encoded)
   * 
   * Note: JavaScript numbers are 64-bit floats and can only safely represent
   * integers up to 2^53-1. Values larger than this may lose precision.
   * 
   * @param value - 64-bit signed integer (as number)
   * @returns this (for chaining)
   */
  sint64(value: number): this;

  /**
   * Write a fixed64 value (little-endian)
   * 
   * Note: JavaScript numbers are 64-bit floats and can only safely represent
   * integers up to 2^53-1. Values larger than this may lose precision.
   * 
   * @param value - 64-bit unsigned integer (as number)
   * @returns this (for chaining)
   */
  fixed64(value: number): this;

  /**
   * Write a sfixed64 value (little-endian)
   * 
   * Note: JavaScript numbers are 64-bit floats and can only safely represent
   * integers up to 2^53-1. Values larger than this may lose precision.
   * 
   * @param value - 64-bit signed integer (as number)
   * @returns this (for chaining)
   */
  sfixed64(value: number): this;

  /**
   * Write a float value (32-bit IEEE 754)
   * 
   * @param value - Float value
   * @returns this (for chaining)
   */
  float(value: number): this;

  /**
   * Write a double value (64-bit IEEE 754)
   * 
   * @param value - Double value
   * @returns this (for chaining)
   */
  double(value: number): this;

  /**
   * Write a bool value
   * 
   * @param value - Boolean value
   * @returns this (for chaining)
   */
  bool(value: boolean): this;

  /**
   * Write a string with length prefix
   * 
   * Format: [length: varint][utf8_data: bytes]
   * Strings are automatically encoded as UTF-8.
   * 
   * @param value - String to write
   * @returns this (for chaining)
   */
  string(value: string): this;

  /**
   * Write bytes with length prefix
   * 
   * Format: [length: varint][data: bytes]
   * 
   * @param value - Byte buffer to write
   * @returns this (for chaining)
   */
  bytes(value: Buffer): this;

  /**
   * Fork for length-delimited operations
   * 
   * Starts a new nested writing context for length-delimited fields.
   * Must be followed by ldelim() to complete the operation.
   * 
   * @returns this (for chaining)
   * 
   * @example
   * ```typescript
   * writer.fork();
   * writer.uint32(1);
   * writer.uint32(2);
   * writer.ldelim();  // Writes length prefix and content
   * ```
   */
  fork(): this;

  /**
   * Complete length-delimited operation
   * 
   * Completes a fork() operation by writing the length prefix
   * and the accumulated content.
   * 
   * @returns this (for chaining)
   * @throws {Error} If no fork to delimit
   */
  ldelim(): this;

  /**
   * Finish writing and get the buffer
   * 
   * Returns the complete buffer containing all written data.
   * The writer remains usable after calling finish().
   * Call reset() if you want to reuse the writer.
   * 
   * @returns Buffer containing all written data
   */
  finish(): Buffer;

  /**
   * Get current buffer size (number of bytes written)
   * 
   * @returns Current size in bytes
   */
  estimatedSize(): number;

  /**
   * Get current length (alias for estimatedSize)
   * 
   * @returns Current length in bytes
   */
  len(): number;

  /**
   * Check if buffer is empty
   * 
   * @returns true if empty
   */
  isEmpty(): boolean;

  /**
   * Reset the writer for reuse
   * 
   * Clears the buffer but keeps allocated capacity to avoid reallocation.
   * 
   * @example
   * ```typescript
   * const writer = Writer.withCapacity(1024);
   * writer.uint32(100);
   * const buffer1 = writer.finish();
   * 
   * writer.reset();  // Reuse without reallocation
   * writer.uint32(200);
   * const buffer2 = writer.finish();
   * ```
   */
  reset(): void;
}

// =============================================================================
// SIMD Batch Processing Functions (Phase 3)
// =============================================================================

/**
 * Batch encode multiple uint32 values using SIMD optimizations
 * 
 * Currently uses optimized scalar implementation. Hardware SIMD (AVX2/NEON)
 * support will be added in a future release.
 * 
 * Performance benefits:
 * - Batch processing reduces function call overhead
 * - Pre-allocated buffer reduces memory allocations
 * - Future SIMD: 4-8x speedup (planned for v1.1)
 * 
 * @param values - Array of uint32 values to encode
 * @returns Buffer containing all encoded varints
 * 
 * @example
 * ```typescript
 * const values = [1, 2, 3, 100, 1000];
 * const encoded = encodeVarintBatchSimd(values);
 * ```
 */
export function encodeVarintBatchSimd(values: number[]): Buffer;

/**
 * Batch decode varints from buffer using SIMD optimizations
 * 
 * Currently uses optimized scalar implementation. Hardware SIMD (AVX2/NEON)
 * support will be added in a future release.
 * 
 * Performance benefits:
 * - Batch parsing reduces overhead
 * - Reduced bounds checking
 * - Cache-friendly sequential access
 * 
 * @param buffer - Buffer containing multiple varints
 * @returns Array of decoded uint32 values
 * 
 * @example
 * ```typescript
 * const values = decodeVarintBatchSimd(buffer);
 * ```
 */
export function decodeVarintBatchSimd(buffer: Buffer): number[];

// =============================================================================
// Parallel Processing Functions (Phase 3)
// =============================================================================

/**
 * Encode multiple varints in parallel
 * 
 * Uses Rayon library for work-stealing parallelism, automatically
 * utilizing all CPU cores.
 * 
 * Performance benefits:
 * - Multi-core parallelism: Near-linear scaling (8 cores ~7.5x)
 * - Lock-free design: Each thread works independently
 * - Work stealing: Dynamic load balancing
 * 
 * Best for:
 * - Large datasets (>1000 values)
 * - CPU-intensive tasks
 * - Multi-core server environments
 * 
 * @param values - Array of int64 values to encode
 * @returns Array of encoded buffers (one per value)
 * 
 * @example
 * ```typescript
 * const values = [1, 2, 3, 100, 1000];
 * const buffers = encodeVarintsParallel(values);
 * // Each buffer contains one encoded varint
 * ```
 */
export function encodeVarintsParallel(values: number[]): Buffer[];

/**
 * Decode multiple varints in parallel
 * 
 * Converts all buffers to owned data, then decodes in parallel.
 * 
 * @param buffers - Array of buffers containing varints
 * @returns Array of decoded int64 values
 * 
 * @example
 * ```typescript
 * const values = decodeVarintsParallel(buffers);
 * ```
 */
export function decodeVarintsParallel(buffers: Buffer[]): number[];

/**
 * Process batch of uint32 varints in parallel using chunking
 * 
 * Splits large datasets into chunks, processes each in parallel,
 * then merges results.
 * 
 * Performance tuning guidelines:
 * - Small datasets (<1000): Use default chunk size 100
 * - Medium datasets (1K-100K): Chunk size 1000
 * - Large datasets (>100K): Chunk size 10000
 * 
 * @param values - Array of uint32 values to process
 * @param chunkSize - Optional chunk size (default: 100)
 * @returns Buffer containing all encoded varints
 * 
 * @example
 * ```typescript
 * // Process 100K values with chunk size 1000
 * const result = processU32BatchParallel(values, 1000);
 * ```
 */
export function processU32BatchParallel(values: number[], chunkSize?: number): Buffer;
