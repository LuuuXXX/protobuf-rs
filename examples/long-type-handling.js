/**
 * Long 类型处理示例
 * Long Type Handling Example
 * 
 * 演示如何处理 64 位整数（Long 类型）。
 * Demonstrates how to handle 64-bit integers (Long types).
 */

const { Reader, Writer } = require('../integration/protobufjs-adapter');

console.log('='.repeat(80));
console.log('Long 类型处理示例 | Long Type Handling Example');
console.log('='.repeat(80));
console.log();

// =============================================================================
// 示例 1: JavaScript 安全整数范围 | Example 1: JavaScript Safe Integer Range
// =============================================================================
console.log('示例 1: JavaScript 的安全整数范围');
console.log('Example 1: JavaScript Safe Integer Range');
console.log('-'.repeat(80));

console.log('JavaScript 安全整数范围 | JavaScript safe integer range:');
console.log('  Number.MAX_SAFE_INTEGER:', Number.MAX_SAFE_INTEGER);
console.log('  Number.MIN_SAFE_INTEGER:', Number.MIN_SAFE_INTEGER);
console.log('  范围 | Range: ±2^53 ≈ ±9 × 10^15');
console.log();

console.log('超出此范围的整数可能失去精度！');
console.log('Integers outside this range may lose precision!');
console.log();

// =============================================================================
// 示例 2: 安全范围内的 64 位整数 | Example 2: 64-bit Integers within Safe Range
// =============================================================================
console.log('示例 2: 编码和解码安全范围内的 64 位整数');
console.log('Example 2: Encoding and Decoding 64-bit Integers within Safe Range');
console.log('-'.repeat(80));

const safeValues = [
  0,
  1,
  42,
  1000000,
  1000000000,           // 10亿 | 1 billion
  1000000000000,        // 1万亿 | 1 trillion
  Number.MAX_SAFE_INTEGER
];

console.log('测试值 | Test values:');
safeValues.forEach(value => {
  const writer = Writer.create();
  writer.uint64(value);
  const buffer = writer.finish();
  
  const reader = Reader.create(buffer);
  const decoded = reader.uint64();
  
  const match = decoded === value ? '✓' : '✗';
  console.log(`  ${match} ${value} -> ${decoded} (${buffer.length} bytes)`);
});
console.log();

// =============================================================================
// 示例 3: 有符号 64 位整数（sint64）| Example 3: Signed 64-bit Integers (sint64)
// =============================================================================
console.log('示例 3: 有符号 64 位整数（sint64，ZigZag 编码）');
console.log('Example 3: Signed 64-bit Integers (sint64, ZigZag encoded)');
console.log('-'.repeat(80));

const signedValues = [
  -1,
  -100,
  -1000000,
  0,
  1,
  100,
  1000000
];

console.log('sint64 编码/解码（ZigZag）| sint64 encoding/decoding (ZigZag):');
signedValues.forEach(value => {
  const writer = Writer.create();
  writer.sint64(value);
  const buffer = writer.finish();
  
  const reader = Reader.create(buffer);
  const decoded = reader.sint64();
  
  const match = decoded === value ? '✓' : '✗';
  console.log(`  ${match} ${value} -> ${decoded} (${buffer.length} bytes)`);
});
console.log();

// =============================================================================
// 示例 4: 固定 64 位整数 | Example 4: Fixed 64-bit Integers
// =============================================================================
console.log('示例 4: 固定 64 位整数（fixed64、sfixed64）');
console.log('Example 4: Fixed 64-bit Integers (fixed64, sfixed64)');
console.log('-'.repeat(80));

const fixedValues = [0, 42, 1000000, Number.MAX_SAFE_INTEGER];

console.log('fixed64（始终 8 字节）| fixed64 (always 8 bytes):');
fixedValues.forEach(value => {
  const writer = Writer.create();
  writer.fixed64(value);
  const buffer = writer.finish();
  
  const reader = Reader.create(buffer);
  const decoded = reader.fixed64();
  
  const match = decoded === value ? '✓' : '✗';
  console.log(`  ${match} ${value} -> ${decoded} (${buffer.length} bytes, always 8)`);
});
console.log();

// =============================================================================
// 示例 5: 常见用例 - 时间戳 | Example 5: Common Use Case - Timestamps
// =============================================================================
console.log('示例 5: 常见用例 - 时间戳（毫秒）');
console.log('Example 5: Common Use Case - Timestamps (milliseconds)');
console.log('-'.repeat(80));

// Unix 时间戳（毫秒）| Unix timestamp (milliseconds)
const now = Date.now();
console.log('当前时间戳 | Current timestamp:', now);
console.log('日期 | Date:', new Date(now).toISOString());

const writer = Writer.create();
writer.uint64(now);
const timestampBuffer = writer.finish();

const reader = Reader.create(timestampBuffer);
const decodedTimestamp = reader.uint64();

console.log('编码大小 | Encoded size:', timestampBuffer.length, 'bytes');
console.log('解码时间戳 | Decoded timestamp:', decodedTimestamp);
console.log('解码日期 | Decoded date:', new Date(decodedTimestamp).toISOString());
console.log('匹配 | Match:', decodedTimestamp === now ? '✓' : '✗');
console.log();

// =============================================================================
// 示例 6: 常见用例 - ID | Example 6: Common Use Case - IDs
// =============================================================================
console.log('示例 6: 常见用例 - 用户 ID、会话 ID');
console.log('Example 6: Common Use Case - User IDs, Session IDs');
console.log('-'.repeat(80));

// message User {
//   uint64 user_id = 1;
//   uint64 session_id = 2;
//   uint32 created_at = 3;
// }

function encodeUser(user) {
  const writer = Writer.create();
  
  if (user.user_id !== undefined) {
    writer.uint32((1 << 3) | 0);
    writer.uint64(user.user_id);
  }
  
  if (user.session_id !== undefined) {
    writer.uint32((2 << 3) | 0);
    writer.uint64(user.session_id);
  }
  
  if (user.created_at !== undefined) {
    writer.uint32((3 << 3) | 0);
    writer.uint32(user.created_at);
  }
  
  return writer.finish();
}

function decodeUser(buffer) {
  const reader = Reader.create(buffer);
  const user = {};
  
  while (reader.pos < reader.len) {
    const tag = reader.uint32();
    const fieldNumber = tag >>> 3;
    
    switch (fieldNumber) {
      case 1:
        user.user_id = reader.uint64();
        break;
      case 2:
        user.session_id = reader.uint64();
        break;
      case 3:
        user.created_at = reader.uint32();
        break;
      default:
        reader.skipType(tag & 7);
    }
  }
  
  return user;
}

const user = {
  user_id: 1234567890123,      // 13 位 | 13 digits
  session_id: 9876543210987,   // 13 位 | 13 digits
  created_at: Math.floor(Date.now() / 1000) // Unix 时间戳（秒）| Unix timestamp (seconds)
};

console.log('原始用户对象 | Original user object:', user);

const userBuffer = encodeUser(user);
console.log('编码大小 | Encoded size:', userBuffer.length, 'bytes');

const decodedUser = decodeUser(userBuffer);
console.log('解码用户对象 | Decoded user object:', decodedUser);
console.log('匹配 | Match:', 
  decodedUser.user_id === user.user_id &&
  decodedUser.session_id === user.session_id &&
  decodedUser.created_at === user.created_at ? '✓' : '✗');
console.log();

// =============================================================================
// 示例 7: 精度问题演示 | Example 7: Precision Issues Demonstration
// =============================================================================
console.log('示例 7: 超出安全范围的精度问题');
console.log('Example 7: Precision Issues beyond Safe Range');
console.log('-'.repeat(80));

console.log('⚠️  警告：这些值超出 JavaScript 安全整数范围！');
console.log('⚠️  Warning: These values are beyond JavaScript safe integer range!');
console.log();

// 这些值可能失去精度 | These values may lose precision
const unsafeValues = [
  Number.MAX_SAFE_INTEGER + 1,
  Number.MAX_SAFE_INTEGER + 2,
  Number.MAX_SAFE_INTEGER + 100,
];

unsafeValues.forEach(value => {
  console.log(`尝试编码 | Attempting to encode: ${value}`);
  console.log(`  在 JavaScript 中 | In JavaScript: ${value}`);
  console.log(`  精度丢失 | Precision lost:`, value !== Math.floor(value) || 
    value === Number.MAX_SAFE_INTEGER + 1);
  console.log();
});

// =============================================================================
// 示例 8: 最佳实践和建议 | Example 8: Best Practices and Recommendations
// =============================================================================
console.log('示例 8: 处理大整数的最佳实践');
console.log('Example 8: Best Practices for Handling Large Integers');
console.log('-'.repeat(80));

console.log('建议 | Recommendations:');
console.log();

console.log('1. 对于大多数应用，使用 uint32 或 int32:');
console.log('   For most applications, use uint32 or int32:');
console.log('   - uint32: 0 到 4,294,967,295 (约 40 亿)');
console.log('   - int32: -2,147,483,648 到 2,147,483,647');
console.log('   - 足够大多数用例（用户 ID、计数器等）');
console.log();

console.log('2. 对于时间戳，使用 uint32（秒）或 uint64（毫秒）:');
console.log('   For timestamps, use uint32 (seconds) or uint64 (milliseconds):');
console.log('   - uint32 秒可表示到 2106 年');
console.log('   - uint64 毫秒在 JavaScript 安全范围内到 2255 年');
console.log();

console.log('3. 对于确实需要大整数的场景:');
console.log('   For scenarios that truly need large integers:');
console.log('   - 使用字符串表示');
console.log('   - 使用 Long 库（如 long.js）');
console.log('   - 等待 protobuf-rs v1.1.0（计划支持 Long）');
console.log();

console.log('4. 测试你的具体用例:');
console.log('   Test your specific use case:');
console.log('   - 确认值在安全范围内');
console.log('   - 如果不确定，添加验证');
console.log();

// 示例验证函数 | Example validation function
function isSafeInteger(value) {
  return Number.isSafeInteger(value);
}

console.log('示例验证 | Example validation:');
console.log('  isSafeInteger(1000000):', isSafeInteger(1000000));
console.log('  isSafeInteger(Number.MAX_SAFE_INTEGER):', isSafeInteger(Number.MAX_SAFE_INTEGER));
console.log('  isSafeInteger(Number.MAX_SAFE_INTEGER + 1):', isSafeInteger(Number.MAX_SAFE_INTEGER + 1));
console.log();

// =============================================================================
// 总结 | Summary
// =============================================================================
console.log('='.repeat(80));
console.log('✅ Long 类型处理示例完成！');
console.log('✅ Long Type Handling Examples Completed!');
console.log('='.repeat(80));
console.log();
console.log('关键要点 | Key Takeaways:');
console.log('1. JavaScript 安全整数范围：±2^53');
console.log('   JavaScript safe integer range: ±2^53');
console.log('2. 大多数应用使用 uint32/int32 就足够');
console.log('   Most applications are fine with uint32/int32');
console.log('3. 在安全范围内，uint64/int64 工作正常');
console.log('   Within safe range, uint64/int64 work fine');
console.log('4. 超出范围的值需要特殊处理');
console.log('   Values beyond range need special handling');
console.log();
