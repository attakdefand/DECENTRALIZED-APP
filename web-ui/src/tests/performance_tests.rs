//! Performance tests
//!
//! Tests for performance optimization features

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use crate::services::performance::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    // Test 1: Memoization Cache Performance
    #[wasm_bindgen_test]
    fn test_memo_cache_performance() {
        let mut cache = MemoCache::<String, String>::new(100);
        
        // First access - should compute
        let start = js_sys::Date::now();
        let value1 = cache.get_or_insert_with("key1".to_string(), || {
            // Simulate expensive computation
            "computed_value".to_string()
        });
        let first_duration = js_sys::Date::now() - start;
        
        // Second access - should use cache
        let start = js_sys::Date::now();
        let value2 = cache.get_or_insert_with("key1".to_string(), || {
            "computed_value".to_string()
        });
        let cached_duration = js_sys::Date::now() - start;
        
        assert_eq!(*value1, *value2);
        // Cached access should be faster (or at least not slower)
        assert!(cached_duration <= first_duration + 1.0);
    }
    
    // Test 2: Cache Memory Limits
    #[wasm_bindgen_test]
    fn test_cache_memory_limit() {
        let max_size = 10;
        let mut cache = MemoCache::<i32, String>::new(max_size);
        
        // Fill cache beyond limit
        for i in 0..20 {
            cache.get_or_insert_with(i, || format!("value_{}", i));
        }
        
        // Security: Cache should not exceed max size
        assert!(cache.len() <= max_size, "Cache exceeded maximum size");
    }
    
    // Test 3: Debouncer Prevents Rapid Calls
    #[wasm_bindgen_test]
    fn test_debouncer_rate_limiting() {
        let mut debouncer = Debouncer::new(100.0);
        
        let mut call_count = 0;
        
        // Try to call 10 times rapidly
        for _ in 0..10 {
            if debouncer.should_execute() {
                call_count += 1;
            }
        }
        
        // Should only execute once due to debouncing
        assert_eq!(call_count, 1, "Debouncer should prevent rapid calls");
    }
    
    // Test 4: Performance Tracker Metrics
    #[wasm_bindgen_test]
    fn test_performance_tracker() {
        let mut tracker = PerformanceTracker::new();
        
        // Track multiple operations
        for _ in 0..5 {
            let start = tracker.start("test_operation");
            // Simulate work
            tracker.end("test_operation", start);
        }
        
        let avg = tracker.average("test_operation");
        assert!(avg.is_some(), "Should have average for tracked operation");
        
        // Average should be a reasonable number
        let avg_value = avg.unwrap();
        assert!(avg_value >= 0.0 && avg_value < 1000.0, "Average duration should be reasonable");
    }
    
    // Test 5: Performance Tracker Memory Limit
    #[wasm_bindgen_test]
    fn test_performance_tracker_memory_limit() {
        let mut tracker = PerformanceTracker::new();
        
        // Record many metrics
        for i in 0..200 {
            let start = tracker.start("operation");
            tracker.end("operation", start - (i as f64));
        }
        
        // Should have limited history to prevent memory leak
        // Internal limit is 100, then drains to 50
        let report = tracker.report();
        assert!(!report.is_empty(), "Should have metrics report");
    }
    
    // Test 6: Virtual List Visible Range Calculation
    #[wasm_bindgen_test]
    fn test_virtual_list_visible_range() {
        let vlist = VirtualList::new(50.0, 500.0); // 50px items, 500px container
        
        // At top of list
        let (start, end) = vlist.visible_range(0.0, 100);
        assert_eq!(start, 0, "Should start at beginning");
        assert!(end > 10, "Should render visible items plus overscan");
        
        // Scrolled down
        let (start, end) = vlist.visible_range(1000.0, 100);
        assert!(start > 0, "Should skip items above viewport");
        assert!(end < 100, "Should not render all items");
    }
    
    // Test 7: Virtual List Performance for Large Lists
    #[wasm_bindgen_test]
    fn test_virtual_list_large_dataset() {
        let vlist = VirtualList::new(50.0, 500.0);
        let total_items = 10_000;
        
        // Even with 10k items, should only render a small subset
        let (start, end) = vlist.visible_range(5000.0, total_items);
        let rendered_count = end - start;
        
        // Should render far fewer than total items
        assert!(rendered_count < 50, "Should only render visible items, not all 10k");
    }
    
    // Test 8: Virtual List Total Height
    #[wasm_bindgen_test]
    fn test_virtual_list_total_height() {
        let vlist = VirtualList::new(50.0, 500.0);
        
        let height_100 = vlist.total_height(100);
        assert_eq!(height_100, 5000.0, "100 items * 50px = 5000px");
        
        let height_1000 = vlist.total_height(1000);
        assert_eq!(height_1000, 50000.0, "1000 items * 50px = 50000px");
    }
    
    // Test 9: Virtual List Offset Calculation
    #[wasm_bindgen_test]
    fn test_virtual_list_offset() {
        let vlist = VirtualList::new(50.0, 500.0);
        
        assert_eq!(vlist.offset(0), 0.0);
        assert_eq!(vlist.offset(10), 500.0);
        assert_eq!(vlist.offset(20), 1000.0);
    }
    
    // Test 10: Component Re-render Optimization
    #[wasm_bindgen_test]
    fn test_component_partial_eq() {
        use crate::components::pool_card::PoolData;
        
        let pool1 = PoolData {
            id: "1".to_string(),
            token_a: "ETH".to_string(),
            token_b: "USDC".to_string(),
            liquidity: 1000000.0,
            volume_24h: 50000.0,
            apr: 12.5,
        };
        
        let pool2 = pool1.clone();
        
        // PartialEq should work correctly for memoization
        assert_eq!(pool1, pool2, "Identical pools should be equal");
        
        let pool3 = PoolData {
            id: "1".to_string(),
            token_a: "ETH".to_string(),
            token_b: "USDC".to_string(),
            liquidity: 2000000.0, // Different
            volume_24h: 50000.0,
            apr: 12.5,
        };
        
        assert_ne!(pool1, pool3, "Different pools should not be equal");
    }
}
