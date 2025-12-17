"use strict";
module.exports = Writer;

let NativeWriter;
try {
    const native = require('../index.node');
    NativeWriter = native.Writer;
} catch (e) {
    NativeWriter = null;
}

function Writer() {
    if (NativeWriter) {
        return new NativeWriter();
    } else {
        // Pure JS fallback (will be replaced in Phase 3)
        this.len = 0;
        this.buf = [];
    }
}

Writer.create = function() {
    return new Writer();
};

Writer._useNative = !!NativeWriter;
