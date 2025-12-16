# ohosTest Structure for protobuf-rs Entry Module

This directory contains the complete HarmonyOS test infrastructure for the protobuf-rs entry module, following the Hypium testing framework standards.

## Directory Structure

```
entry/src/ohosTest/
├── ets/
│   ├── test/
│   │   ├── Ability.test.ets      # Main test cases
│   │   ├── List.test.ets         # Test suite entry point
│   │   ├── user.d.ts             # TypeScript type declarations
│   │   └── user.js               # Protobuf message implementation
│   ├── testability/
│   │   ├── TestAbility.ets       # Test UIAbility
│   │   └── pages/
│   │       └── Index.ets         # Test UI page
│   └── testrunner/
│       └── OpenHarmonyTestRunner.ts  # Test runner
├── resources/
│   └── base/
│       ├── element/
│       │   ├── color.json        # Color resources
│       │   └── string.json       # String resources
│       └── profile/
│           └── test_pages.json   # Page configuration
└── module.json5                   # Test module configuration
```

## Test Coverage

### Basic Functionality Tests
1. **UserLoginResponse Creation** - Tests message object creation
2. **Message Encoding** - Tests protobuf encoding functionality
3. **Message Decoding** - Tests protobuf decoding functionality
4. **String Types** - Tests string field handling with Unicode support
5. **Boolean Types** - Tests boolean field handling
6. **Number Types** - Tests various numeric types (uint32, int32, etc.)
7. **BigInt/Long Support** - Tests 64-bit integer handling
8. **Uint8Array/Bytes** - Tests binary data handling

### Performance Tests
1. **Batch Create** - Creates 2000 message instances
2. **Batch Encode** - Encodes 2000 messages
3. **Batch Decode** - Decodes 2000 messages
4. **Round-trip Test** - Performs 1000 encode/decode cycles

## Running Tests

To run the tests in DevEco Studio:

1. Open the project in DevEco Studio
2. Select the device or emulator
3. Right-click on `entry/src/ohosTest` directory
4. Select "Run 'entry_test'" or use the test configuration

## Test Data Model

The `UserLoginResponse` message includes the following fields:
- `userId` (string) - User identifier
- `userName` (string) - User name
- `isActive` (boolean) - User active status
- `timestamp` (int64) - Timestamp value
- `sessionToken` (bytes) - Binary session token

## Performance Expectations

All performance tests should complete within 5 seconds on standard HarmonyOS devices:
- Batch create: ~400-1000 ops/ms
- Batch encode: ~400-1000 ops/ms
- Batch decode: ~400-1000 ops/ms
- Round-trip: ~200-500 ops/ms

## Dependencies

- `@ohos/hypium` - HarmonyOS testing framework
- `protobuf-rs library` - The main protobuf library module

## Notes

1. The tests import Reader/Writer from the library module using relative paths
2. Long type support is optional and handled gracefully
3. All tests include performance metrics logging via hilog
4. Tests are compatible with both debug and release builds
