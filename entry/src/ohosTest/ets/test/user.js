/**
 * UserLoginResponse protobuf message implementation
 * This file implements the protobuf message encoding/decoding for testing
 */

// Import Long support for 64-bit integers
let Long;
try {
  Long = require('long');
} catch (e) {
  // Long is optional, will use number for compatibility
  Long = null;
}

/**
 * UserLoginResponse message constructor
 * @constructor
 * @param {Object} [properties] Properties to set
 */
function UserLoginResponse(properties) {
  if (properties) {
    for (let keys = Object.keys(properties), i = 0; i < keys.length; ++i) {
      if (properties[keys[i]] != null) {
        this[keys[i]] = properties[keys[i]];
      }
    }
  }
}

/**
 * UserLoginResponse userId
 * @member {string} userId
 * @memberof UserLoginResponse
 * @instance
 */
UserLoginResponse.prototype.userId = "";

/**
 * UserLoginResponse userName
 * @member {string} userName
 * @memberof UserLoginResponse
 * @instance
 */
UserLoginResponse.prototype.userName = "";

/**
 * UserLoginResponse isActive
 * @member {boolean} isActive
 * @memberof UserLoginResponse
 * @instance
 */
UserLoginResponse.prototype.isActive = false;

/**
 * UserLoginResponse timestamp
 * @member {number|Long} timestamp
 * @memberof UserLoginResponse
 * @instance
 */
UserLoginResponse.prototype.timestamp = Long ? Long.fromBits(0, 0, false) : 0;

/**
 * UserLoginResponse sessionToken
 * @member {Uint8Array} sessionToken
 * @memberof UserLoginResponse
 * @instance
 */
UserLoginResponse.prototype.sessionToken = null;

/**
 * Creates a new UserLoginResponse instance
 * @function create
 * @memberof UserLoginResponse
 * @static
 * @param {Object} [properties] Properties to set
 * @returns {UserLoginResponse} UserLoginResponse instance
 */
UserLoginResponse.create = function create(properties) {
  return new UserLoginResponse(properties);
};

/**
 * Encodes a UserLoginResponse message
 * @function encode
 * @memberof UserLoginResponse
 * @static
 * @param {Object} message UserLoginResponse message or plain object to encode
 * @param {Object} [writer] Writer to encode to
 * @returns {Object} Writer
 */
UserLoginResponse.encode = function encode(message, writer) {
  if (!writer) {
    writer = { _data: [], uint32: function(v) { this._data.push({type: 'uint32', value: v}); return this; }, 
      string: function(v) { this._data.push({type: 'string', value: v}); return this; },
      bool: function(v) { this._data.push({type: 'bool', value: v}); return this; },
      int64: function(v) { this._data.push({type: 'int64', value: v}); return this; },
      bytes: function(v) { this._data.push({type: 'bytes', value: v}); return this; },
      finish: function() { return this._data; }
    };
  }
  
  if (message.userId != null && Object.hasOwnProperty.call(message, "userId")) {
    writer.uint32(/* id 1, wireType 2 =*/10).string(message.userId);
  }
  if (message.userName != null && Object.hasOwnProperty.call(message, "userName")) {
    writer.uint32(/* id 2, wireType 2 =*/18).string(message.userName);
  }
  if (message.isActive != null && Object.hasOwnProperty.call(message, "isActive")) {
    writer.uint32(/* id 3, wireType 0 =*/24).bool(message.isActive);
  }
  if (message.timestamp != null && Object.hasOwnProperty.call(message, "timestamp")) {
    writer.uint32(/* id 4, wireType 0 =*/32).int64(message.timestamp);
  }
  if (message.sessionToken != null && Object.hasOwnProperty.call(message, "sessionToken")) {
    writer.uint32(/* id 5, wireType 2 =*/42).bytes(message.sessionToken);
  }
  
  return writer;
};

/**
 * Decodes a UserLoginResponse message
 * @function decode
 * @memberof UserLoginResponse
 * @static
 * @param {Object} reader Reader or buffer to decode from
 * @param {number} [length] Message length if known beforehand
 * @returns {UserLoginResponse} UserLoginResponse
 */
UserLoginResponse.decode = function decode(reader, length) {
  const message = new UserLoginResponse();
  const end = length !== undefined ? reader.pos + length : reader.len;
  
  while (reader.pos < end) {
    const tag = reader.uint32();
    switch (tag >>> 3) {
      case 1:
        message.userId = reader.string();
        break;
      case 2:
        message.userName = reader.string();
        break;
      case 3:
        message.isActive = reader.bool();
        break;
      case 4:
        message.timestamp = reader.int64();
        break;
      case 5:
        message.sessionToken = reader.bytes();
        break;
      default:
        reader.skipType(tag & 7);
        break;
    }
  }
  
  return message;
};

/**
 * Creates a UserLoginResponse from a plain object
 * @function fromObject
 * @memberof UserLoginResponse
 * @static
 * @param {Object} object Plain object
 * @returns {UserLoginResponse} UserLoginResponse
 */
UserLoginResponse.fromObject = function fromObject(object) {
  if (object instanceof UserLoginResponse) {
    return object;
  }
  const message = new UserLoginResponse();
  
  if (object.userId != null) {
    message.userId = String(object.userId);
  }
  if (object.userName != null) {
    message.userName = String(object.userName);
  }
  if (object.isActive != null) {
    message.isActive = Boolean(object.isActive);
  }
  if (object.timestamp != null) {
    if (Long) {
      message.timestamp = Long.fromValue(object.timestamp);
    } else {
      message.timestamp = typeof object.timestamp === 'number' ? object.timestamp : parseInt(object.timestamp, 10);
    }
  }
  if (object.sessionToken != null) {
    if (typeof object.sessionToken === "string") {
      message.sessionToken = new Uint8Array(Buffer.from(object.sessionToken, "base64"));
    } else if (object.sessionToken.length) {
      message.sessionToken = object.sessionToken;
    }
  }
  
  return message;
};

/**
 * Converts a UserLoginResponse to a plain object
 * @function toObject
 * @memberof UserLoginResponse
 * @static
 * @param {UserLoginResponse} message UserLoginResponse
 * @param {Object} [options] Conversion options
 * @returns {Object} Plain object
 */
UserLoginResponse.toObject = function toObject(message, options) {
  if (!options) {
    options = {};
  }
  const object = {};
  
  if (options.defaults) {
    object.userId = "";
    object.userName = "";
    object.isActive = false;
    object.timestamp = Long ? Long.ZERO : 0;
    object.sessionToken = null;
  }
  
  if (message.userId != null && message.hasOwnProperty("userId")) {
    object.userId = message.userId;
  }
  if (message.userName != null && message.hasOwnProperty("userName")) {
    object.userName = message.userName;
  }
  if (message.isActive != null && message.hasOwnProperty("isActive")) {
    object.isActive = message.isActive;
  }
  if (message.timestamp != null && message.hasOwnProperty("timestamp")) {
    if (typeof message.timestamp === "number") {
      object.timestamp = message.timestamp;
    } else {
      object.timestamp = Long ? Long.fromValue(message.timestamp) : message.timestamp;
    }
  }
  if (message.sessionToken != null && message.hasOwnProperty("sessionToken")) {
    object.sessionToken = message.sessionToken;
  }
  
  return object;
};

module.exports = { UserLoginResponse };
