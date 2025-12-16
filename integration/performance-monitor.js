// Copyright (c) 2024 LuuuXXX
// Licensed under the BSD-3-Clause License.
// See LICENSE file in the project root for full license information.

/**
 * Performance Monitor
 * 
 * Utility for tracking and comparing operation performance.
 * Useful for benchmarking Rust vs JavaScript implementations.
 */

class PerformanceMonitor {
  /**
   * Create a new performance monitor
   * @param {string} name - Name of the benchmark
   */
  constructor(name = 'Performance Monitor') {
    this.name = name;
    this.metrics = new Map();
  }

  /**
   * Record an operation's execution time
   * @param {string} operationName - Name of the operation
   * @param {number} timeMs - Execution time in milliseconds
   */
  record(operationName, timeMs) {
    if (!this.metrics.has(operationName)) {
      this.metrics.set(operationName, {
        count: 0,
        totalTime: 0,
        minTime: Infinity,
        maxTime: -Infinity,
        times: []
      });
    }

    const metric = this.metrics.get(operationName);
    metric.count++;
    metric.totalTime += timeMs;
    metric.minTime = Math.min(metric.minTime, timeMs);
    metric.maxTime = Math.max(metric.maxTime, timeMs);
    metric.times.push(timeMs);
  }

  /**
   * Get statistics for an operation
   * @param {string} operationName - Name of the operation
   * @returns {Object|null} Statistics object or null if not found
   */
  getStats(operationName) {
    const metric = this.metrics.get(operationName);
    if (!metric) {
      return null;
    }

    const sorted = [...metric.times].sort((a, b) => a - b);
    const median = sorted.length % 2 === 0
      ? (sorted[sorted.length / 2 - 1] + sorted[sorted.length / 2]) / 2
      : sorted[Math.floor(sorted.length / 2)];

    return {
      count: metric.count,
      totalTime: metric.totalTime,
      averageTime: metric.totalTime / metric.count,
      minTime: metric.minTime,
      maxTime: metric.maxTime,
      medianTime: median
    };
  }

  /**
   * Compare two operations and return speedup factor
   * @param {string} baseOperation - Baseline operation name
   * @param {string} compareOperation - Operation to compare
   * @returns {number|null} Speedup factor (positive = faster, negative = slower) or null if not found
   */
  compare(baseOperation, compareOperation) {
    const baseStats = this.getStats(baseOperation);
    const compareStats = this.getStats(compareOperation);

    if (!baseStats || !compareStats) {
      return null;
    }

    return baseStats.averageTime / compareStats.averageTime;
  }

  /**
   * Generate a formatted performance report
   */
  report() {
    console.log('\n' + '='.repeat(80));
    console.log(this.name);
    console.log('='.repeat(80));

    if (this.metrics.size === 0) {
      console.log('No metrics recorded.');
      return;
    }

    // Print individual operation stats
    console.log('\nOperation Statistics:');
    console.log('-'.repeat(80));
    console.log(
      'Operation'.padEnd(25) +
      'Count'.padEnd(10) +
      'Avg (ms)'.padEnd(12) +
      'Min (ms)'.padEnd(12) +
      'Max (ms)'.padEnd(12) +
      'Median (ms)'
    );
    console.log('-'.repeat(80));

    for (const [name, _] of this.metrics) {
      const stats = this.getStats(name);
      console.log(
        name.padEnd(25) +
        stats.count.toString().padEnd(10) +
        stats.averageTime.toFixed(4).padEnd(12) +
        stats.minTime.toFixed(4).padEnd(12) +
        stats.maxTime.toFixed(4).padEnd(12) +
        stats.medianTime.toFixed(4)
      );
    }

    // Auto-detect and compare rust vs js operations
    const operations = Array.from(this.metrics.keys());
    const rustOps = operations.filter(op => op.toLowerCase().includes('rust'));
    const jsOps = operations.filter(op => op.toLowerCase().includes('js') || op.toLowerCase().includes('javascript'));

    if (rustOps.length > 0 && jsOps.length > 0) {
      console.log('\n' + '-'.repeat(80));
      console.log('Performance Comparisons:');
      console.log('-'.repeat(80));

      // Try to match operations by name (removing rust/js prefix)
      for (const rustOp of rustOps) {
        const baseName = rustOp.toLowerCase().replace(/^rust[-_]?/, '');
        const matchingJsOp = jsOps.find(jsOp => {
          const jsBaseName = jsOp.toLowerCase().replace(/^(js|javascript)[-_]?/, '');
          return jsBaseName === baseName;
        });

        if (matchingJsOp) {
          const speedup = this.compare(matchingJsOp, rustOp);
          if (speedup !== null) {
            const rustStats = this.getStats(rustOp);
            const jsStats = this.getStats(matchingJsOp);
            console.log(
              `${rustOp} vs ${matchingJsOp}:`.padEnd(50) +
              `${speedup.toFixed(2)}x faster`.padStart(15) +
              ` (${jsStats.averageTime.toFixed(4)}ms → ${rustStats.averageTime.toFixed(4)}ms)`
            );
          }
        }
      }
    }

    console.log('='.repeat(80) + '\n');
  }

  /**
   * Reset all metrics
   */
  reset() {
    this.metrics.clear();
  }

  /**
   * Get all operation names
   * @returns {Array<string>} Array of operation names
   */
  getOperations() {
    return Array.from(this.metrics.keys());
  }
}

module.exports = PerformanceMonitor;
