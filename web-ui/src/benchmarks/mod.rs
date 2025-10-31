//! Performance benchmarks
//!
//! Comprehensive benchmark suite for measuring and validating performance optimizations

use wasm_bindgen::prelude::*;
use std::collections::HashMap;

/// Benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_ms: f64,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub ops_per_sec: f64,
}

impl BenchmarkResult {
    pub fn new(name: String, iterations: usize, durations: Vec<f64>) -> Self {
        let total_ms: f64 = durations.iter().sum();
        let avg_ms = total_ms / iterations as f64;
        let min_ms = durations.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_ms = durations.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let ops_per_sec = if avg_ms > 0.0 {
            1000.0 / avg_ms
        } else {
            0.0
        };
        
        Self {
            name,
            iterations,
            total_ms,
            avg_ms,
            min_ms,
            max_ms,
            ops_per_sec,
        }
    }
    
    pub fn report(&self) -> String {
        format!(
            "{}\n  Iterations: {}\n  Total: {:.2}ms\n  Average: {:.4}ms\n  Min: {:.4}ms\n  Max: {:.4}ms\n  Ops/sec: {:.0}",
            self.name, self.iterations, self.total_ms, self.avg_ms, self.min_ms, self.max_ms, self.ops_per_sec
        )
    }
}

/// Performance benchmark suite
pub struct BenchmarkSuite {
    results: Vec<BenchmarkResult>,
}

impl BenchmarkSuite {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }
    
    /// Run a benchmark with specified iterations
    pub fn bench<F>(&mut self, name: &str, iterations: usize, mut operation: F)
    where
        F: FnMut(),
    {
        let mut durations = Vec::with_capacity(iterations);
        
        // Warmup
        for _ in 0..10 {
            operation();
        }
        
        // Actual benchmark
        for _ in 0..iterations {
            let start = js_sys::Date::now();
            operation();
            let duration = js_sys::Date::now() - start;
            durations.push(duration);
        }
        
        let result = BenchmarkResult::new(name.to_string(), iterations, durations);
        web_sys::console::log_1(&result.report().into());
        self.results.push(result);
    }
    
    /// Generate full report
    pub fn report(&self) -> String {
        let mut report = String::from("=== Performance Benchmark Results ===\n\n");
        
        for result in &self.results {
            report.push_str(&result.report());
            report.push_str("\n\n");
        }
        
        report.push_str(&format!("Total benchmarks: {}\n", self.results.len()));
        report
    }
    
    /// Get all results
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }
}

impl Default for BenchmarkSuite {
    fn default() -> Self {
        Self::new()
    }
}

/// Run all performance benchmarks
#[wasm_bindgen]
pub fn run_performance_benchmarks() {
    use crate::services::performance::*;
    use crate::services::cache::CacheService;
    use crate::services::throttle::ThrottleService;
    
    web_sys::console::log_1(&"Starting performance benchmarks...".into());
    
    let mut suite = BenchmarkSuite::new();
    
    // Benchmark 1: MemoCache performance
    suite.bench("MemoCache - Cache Hit", 1000, || {
        let mut cache = MemoCache::<String, i32>::new(100);
        cache.get_or_insert_with("key".to_string(), || 42);
        // Second access should be cached
        let _ = cache.get_or_insert_with("key".to_string(), || 99);
    });
    
    // Benchmark 2: MemoCache vs No Cache
    suite.bench("MemoCache - Cache Miss", 1000, || {
        let mut cache = MemoCache::<i32, String>::new(100);
        // Always miss cache
        for i in 0..10 {
            cache.get_or_insert_with(i, || format!("value_{}", i));
        }
    });
    
    // Benchmark 3: CacheService localStorage
    suite.bench("CacheService - localStorage", 100, || {
        let cache = CacheService::new();
        let _ = cache.set_local("bench_key", &"test_value", 60000.0);
        let _: Option<String> = cache.get_local("bench_key");
    });
    
    // Benchmark 4: CacheService memory
    suite.bench("CacheService - memory", 1000, || {
        let mut cache = CacheService::new();
        let _ = cache.set_memory("bench_key", "test_value".to_string(), 60000.0);
        let _: Option<String> = cache.get_memory("bench_key");
    });
    
    // Benchmark 5: Throttle check
    suite.bench("ThrottleService - is_allowed", 1000, || {
        let mut throttle = ThrottleService::new();
        throttle.configure_limit("bench", 1000, 1000.0);
        let _ = throttle.is_allowed("bench");
    });
    
    // Benchmark 6: Virtual List calculation
    suite.bench("VirtualList - visible_range", 1000, || {
        let vlist = VirtualList::new(50.0, 500.0);
        let _ = vlist.visible_range(1000.0, 10000);
    });
    
    // Benchmark 7: Debouncer
    suite.bench("Debouncer - should_execute", 1000, || {
        let mut debouncer = Debouncer::new(100.0);
        let _ = debouncer.should_execute();
    });
    
    // Benchmark 8: PerformanceTracker
    suite.bench("PerformanceTracker - start/end", 1000, || {
        let mut tracker = PerformanceTracker::new();
        let start = tracker.start("test");
        tracker.end("test", start);
    });
    
    // Benchmark 9: Serialization
    suite.bench("JSON Serialization", 1000, || {
        use crate::components::pool_card::PoolData;
        let pool = PoolData {
            id: "1".to_string(),
            token_a: "ETH".to_string(),
            token_b: "USDC".to_string(),
            liquidity: 1000000.0,
            volume_24h: 50000.0,
            apr: 12.5,
        };
        let _ = serde_json::to_string(&pool);
    });
    
    // Benchmark 10: Deserialization
    suite.bench("JSON Deserialization", 1000, || {
        use crate::components::pool_card::PoolData;
        let json = r#"{"id":"1","token_a":"ETH","token_b":"USDC","liquidity":1000000.0,"volume_24h":50000.0,"apr":12.5}"#;
        let _: Result<PoolData, _> = serde_json::from_str(json);
    });
    
    // Print final report
    web_sys::console::log_1(&suite.report().into());
    web_sys::console::log_1(&"Benchmarks complete!".into());
}

/// Compare two benchmark results
pub fn compare_benchmarks(baseline: &BenchmarkResult, current: &BenchmarkResult) -> String {
    let improvement = ((baseline.avg_ms - current.avg_ms) / baseline.avg_ms) * 100.0;
    
    format!(
        "Comparison: {} vs {}\n  Baseline: {:.4}ms\n  Current: {:.4}ms\n  Change: {:.2}% {}",
        baseline.name,
        current.name,
        baseline.avg_ms,
        current.avg_ms,
        improvement.abs(),
        if improvement > 0.0 { "faster ✓" } else { "slower ✗" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_benchmark_result_creation() {
        let durations = vec![10.0, 20.0, 30.0];
        let result = BenchmarkResult::new("test".to_string(), 3, durations);
        
        assert_eq!(result.iterations, 3);
        assert_eq!(result.total_ms, 60.0);
        assert_eq!(result.avg_ms, 20.0);
        assert_eq!(result.min_ms, 10.0);
        assert_eq!(result.max_ms, 30.0);
        assert!(result.ops_per_sec > 0.0);
    }
    
    #[wasm_bindgen_test]
    fn test_benchmark_suite() {
        let mut suite = BenchmarkSuite::new();
        
        suite.bench("simple test", 10, || {
            let _ = 1 + 1;
        });
        
        assert_eq!(suite.results().len(), 1);
        assert_eq!(suite.results()[0].iterations, 10);
    }
    
    #[wasm_bindgen_test]
    fn test_benchmark_comparison() {
        let baseline = BenchmarkResult::new(
            "baseline".to_string(),
            100,
            vec![10.0; 100]
        );
        
        let current = BenchmarkResult::new(
            "current".to_string(),
            100,
            vec![5.0; 100]
        );
        
        let comparison = compare_benchmarks(&baseline, &current);
        assert!(comparison.contains("faster"));
    }
    
    #[wasm_bindgen_test]
    fn test_benchmark_report() {
        let result = BenchmarkResult::new(
            "test".to_string(),
            100,
            vec![10.0; 100]
        );
        
        let report = result.report();
        assert!(report.contains("test"));
        assert!(report.contains("100"));
    }
}
