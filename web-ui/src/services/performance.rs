//! Performance optimization utilities
//!
//! Provides utilities for:
//! - Component memoization
//! - Expensive computation caching
//! - Debouncing and throttling
//! - Memory-efficient data structures

use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

/// Memoization cache for expensive computations
pub struct MemoCache<K, V> {
    cache: HashMap<K, Rc<V>>,
    max_size: usize,
}

impl<K: Hash + Eq, V> MemoCache<K, V> {
    /// Create a new memoization cache with maximum size
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size),
            max_size,
        }
    }
    
    /// Get or compute a value
    pub fn get_or_insert_with<F>(&mut self, key: K, compute: F) -> Rc<V>
    where
        F: FnOnce() -> V,
    {
        if let Some(value) = self.cache.get(&key) {
            return Rc::clone(value);
        }
        
        // Security: Prevent unbounded memory growth
        if self.cache.len() >= self.max_size {
            // Simple eviction: clear oldest entries
            if let Some(first_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&first_key);
            }
        }
        
        let value = Rc::new(compute());
        self.cache.insert(key, Rc::clone(&value));
        value
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    
    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    
    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

/// Debouncer for delaying function execution
pub struct Debouncer {
    last_call: f64,
    delay_ms: f64,
}

impl Debouncer {
    /// Create a new debouncer with specified delay
    pub fn new(delay_ms: f64) -> Self {
        Self {
            last_call: 0.0,
            delay_ms,
        }
    }
    
    /// Check if enough time has passed since last call
    pub fn should_execute(&mut self) -> bool {
        let now = js_sys::Date::now();
        let elapsed = now - self.last_call;
        
        if elapsed >= self.delay_ms {
            self.last_call = now;
            true
        } else {
            false
        }
    }
    
    /// Reset the debouncer
    pub fn reset(&mut self) {
        self.last_call = 0.0;
    }
}

/// Performance metrics tracker
pub struct PerformanceTracker {
    metrics: HashMap<String, Vec<f64>>,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
        }
    }
    
    /// Start tracking an operation
    pub fn start(&self, _operation: &str) -> f64 {
        js_sys::Date::now()
    }
    
    /// End tracking and record duration
    pub fn end(&mut self, operation: &str, start_time: f64) {
        let duration = js_sys::Date::now() - start_time;
        
        self.metrics
            .entry(operation.to_string())
            .or_insert_with(Vec::new)
            .push(duration);
        
        // Security: Limit metrics history to prevent memory leak
        if let Some(durations) = self.metrics.get_mut(operation) {
            if durations.len() > 100 {
                durations.drain(0..50); // Keep only recent 50
            }
        }
    }
    
    /// Get average duration for an operation
    pub fn average(&self, operation: &str) -> Option<f64> {
        self.metrics.get(operation).and_then(|durations| {
            if durations.is_empty() {
                None
            } else {
                let sum: f64 = durations.iter().sum();
                Some(sum / durations.len() as f64)
            }
        })
    }
    
    /// Get metrics report
    pub fn report(&self) -> String {
        let mut report = String::from("Performance Metrics:\n");
        
        for (operation, durations) in &self.metrics {
            if !durations.is_empty() {
                let avg = durations.iter().sum::<f64>() / durations.len() as f64;
                report.push_str(&format!("  {}: {:.2}ms avg\n", operation, avg));
            }
        }
        
        report
    }
}

impl Default for PerformanceTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Lazy value that computes on first access
pub struct Lazy<T, F> {
    value: Option<T>,
    init: Option<F>,
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    /// Create a new lazy value
    pub fn new(init: F) -> Self {
        Self {
            value: None,
            init: Some(init),
        }
    }
    
    /// Get the value, computing if necessary
    pub fn get(&mut self) -> &T {
        if self.value.is_none() {
            let init = self.init.take().expect("Lazy already initialized");
            self.value = Some(init());
        }
        self.value.as_ref().unwrap()
    }
}

/// Virtual list helper for rendering large lists efficiently
pub struct VirtualList {
    item_height: f64,
    container_height: f64,
    overscan: usize,
}

impl VirtualList {
    pub fn new(item_height: f64, container_height: f64) -> Self {
        Self {
            item_height,
            container_height,
            overscan: 3, // Render 3 extra items above and below
        }
    }
    
    /// Calculate which items should be rendered based on scroll position
    pub fn visible_range(&self, scroll_top: f64, total_items: usize) -> (usize, usize) {
        let visible_count = (self.container_height / self.item_height).ceil() as usize;
        
        let start_index = (scroll_top / self.item_height).floor() as usize;
        let start = start_index.saturating_sub(self.overscan);
        
        let end = (start_index + visible_count + self.overscan).min(total_items);
        
        (start, end)
    }
    
    /// Calculate total height of virtual list
    pub fn total_height(&self, total_items: usize) -> f64 {
        self.item_height * total_items as f64
    }
    
    /// Calculate offset for visible items
    pub fn offset(&self, start_index: usize) -> f64 {
        self.item_height * start_index as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_memo_cache_basic() {
        let mut cache = MemoCache::<String, i32>::new(10);
        
        let value1 = cache.get_or_insert_with("key1".to_string(), || 42);
        assert_eq!(*value1, 42);
        
        let value2 = cache.get_or_insert_with("key1".to_string(), || 99);
        assert_eq!(*value2, 42); // Should return cached value
    }
    
    #[wasm_bindgen_test]
    fn test_memo_cache_eviction() {
        let mut cache = MemoCache::<String, i32>::new(2);
        
        cache.get_or_insert_with("key1".to_string(), || 1);
        cache.get_or_insert_with("key2".to_string(), || 2);
        
        assert_eq!(cache.len(), 2);
        
        // This should trigger eviction
        cache.get_or_insert_with("key3".to_string(), || 3);
        assert_eq!(cache.len(), 2); // Still max size
    }
    
    #[wasm_bindgen_test]
    fn test_debouncer() {
        let mut debouncer = Debouncer::new(100.0);
        
        // First call should execute
        assert!(debouncer.should_execute());
        
        // Immediate second call should not execute
        assert!(!debouncer.should_execute());
    }
    
    #[wasm_bindgen_test]
    fn test_performance_tracker() {
        let mut tracker = PerformanceTracker::new();
        
        let start = tracker.start("test_operation");
        tracker.end("test_operation", start);
        
        assert!(tracker.average("test_operation").is_some());
    }
    
    #[wasm_bindgen_test]
    fn test_lazy_value() {
        let mut lazy = Lazy::new(|| {
            42
        });
        
        assert_eq!(*lazy.get(), 42);
        assert_eq!(*lazy.get(), 42); // Should return same value
    }
    
    #[wasm_bindgen_test]
    fn test_virtual_list() {
        let vlist = VirtualList::new(50.0, 500.0);
        
        // Should render about 10 items (500/50) plus overscan
        let (start, end) = vlist.visible_range(0.0, 100);
        
        assert_eq!(start, 0);
        assert!(end > 10); // Should include overscan
        
        // Test total height
        assert_eq!(vlist.total_height(100), 5000.0);
    }
    
    #[wasm_bindgen_test]
    fn test_virtual_list_scrolled() {
        let vlist = VirtualList::new(50.0, 500.0);
        
        // Scroll down 250px (5 items)
        let (start, end) = vlist.visible_range(250.0, 100);
        
        assert!(start < 5); // Should include overscan before
        assert!(end > 15); // Should include items after visible area
    }
}
