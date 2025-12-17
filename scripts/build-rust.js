const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

// Check environment variable to allow skipping build
if (process.env.PROTOBUF_NO_RUST === '1') {
    console.log('[protobuf.js] PROTOBUF_NO_RUST is set, skipping Rust build');
    process.exit(0);
}

// Check if we already have a precompiled binary
const targetFile = path.join(__dirname, '..', 'index.node');
if (fs.existsSync(targetFile)) {
    console.log('[protobuf.js] Native module already exists');
    process.exit(0);
}

// Check if Cargo is available
try {
    execSync('cargo --version', { stdio: 'pipe' });
} catch (err) {
    console.warn('[protobuf.js] Cargo not found, skipping Rust build (will use JS fallback)');
    process.exit(0);
}

// Build the Rust module
try {
    console.log('[protobuf.js] Building Rust native module...');
    const rustDir = path.join(__dirname, '..', 'rust');
    execSync('cargo build --release', { 
        cwd: rustDir,
        stdio: 'inherit' 
    });
    
    // Determine the library extension based on platform
    let libExt = '.node';
    let libPrefix = 'lib';
    if (process.platform === 'win32') {
        libExt = '.dll';
        libPrefix = '';
    } else if (process.platform === 'darwin') {
        libExt = '.dylib';
    } else {
        libExt = '.so';
    }
    
    // Find the compiled library
    const sourceFile = path.join(rustDir, 'target', 'release', `${libPrefix}protobuf_rs${libExt}`);
    
    if (fs.existsSync(sourceFile)) {
        // Copy and rename to .node
        fs.copyFileSync(sourceFile, targetFile);
        console.log('[protobuf.js] ✅ Native module built successfully');
    } else {
        console.warn('[protobuf.js] Build completed but library not found, will use JS fallback');
    }
} catch (err) {
    console.warn('[protobuf.js] Rust build failed, will use JS fallback');
    console.warn('[protobuf.js] Error:', err.message);
    process.exit(0); // Exit successfully to not break npm install
}
