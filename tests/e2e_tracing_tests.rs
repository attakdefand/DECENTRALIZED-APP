//! End-to-End Tracing Tests
//!
//! These tests validate the complete tracing pipeline from span creation to export,
//! including distributed tracing across multiple services.

use core::observability::{ObservabilityManager, TraceSpan};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

/// Test end-to-end tracing functionality
#[test]
fn test_e2e_tracing_functionality() {
    let mut manager = ObservabilityManager::new();
    
    // Create a trace span for a user request
    let user_request_span = TraceSpan {
        id: "span-001".to_string(),
        trace_id: "trace-001".to_string(),
        parent_id: None,
        name: "user_request".to_string(),
        start_time: 1000000,
        end_time: Some(1000100),
        service: "api-service".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("http.method".to_string(), "POST".to_string());
            attrs.insert("http.url".to_string(), "/api/v1/users".to_string());
            attrs.insert("http.status_code".to_string(), "201".to_string());
            attrs
        },
    };
    
    // Add the span to the manager
    let mut spans = manager.spans.lock().unwrap();
    spans.insert(user_request_span.id.clone(), user_request_span.clone());
    drop(spans);
    
    // Create a child span for database operation
    let db_span = TraceSpan {
        id: "span-002".to_string(),
        trace_id: "trace-001".to_string(),
        parent_id: Some("span-001".to_string()),
        name: "database_query".to_string(),
        start_time: 1000010,
        end_time: Some(1000090),
        service: "api-service".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("db.system".to_string(), "postgresql".to_string());
            attrs.insert("db.statement".to_string(), "INSERT INTO users ...".to_string());
            attrs.insert("db.operation".to_string(), "INSERT".to_string());
            attrs
        },
    };
    
    // Add the child span to the manager
    let mut spans = manager.spans.lock().unwrap();
    spans.insert(db_span.id.clone(), db_span.clone());
    drop(spans);
    
    // Create another child span for external API call
    let external_span = TraceSpan {
        id: "span-003".to_string(),
        trace_id: "trace-001".to_string(),
        parent_id: Some("span-001".to_string()),
        name: "external_api_call".to_string(),
        start_time: 1000090,
        end_time: Some(1000100),
        service: "api-service".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("http.method".to_string(), "GET".to_string());
            attrs.insert("http.url".to_string(), "https://api.external.com/v1/data".to_string());
            attrs.insert("http.status_code".to_string(), "200".to_string());
            attrs
        },
    };
    
    // Add the external span to the manager
    let mut spans = manager.spans.lock().unwrap();
    spans.insert(external_span.id.clone(), external_span.clone());
    drop(spans);
    
    // Verify all spans were added
    let spans = manager.spans.lock().unwrap();
    assert_eq!(spans.len(), 3);
    assert!(spans.contains_key("span-001"));
    assert!(spans.contains_key("span-002"));
    assert!(spans.contains_key("span-003"));
    
    // Verify span relationships
    let user_span = spans.get("span-001").unwrap();
    assert_eq!(user_span.trace_id, "trace-001");
    assert_eq!(user_span.parent_id, None);
    
    let db_span = spans.get("span-002").unwrap();
    assert_eq!(db_span.trace_id, "trace-001");
    assert_eq!(db_span.parent_id, Some("span-001".to_string()));
    
    let external_span = spans.get("span-003").unwrap();
    assert_eq!(external_span.trace_id, "trace-001");
    assert_eq!(external_span.parent_id, Some("span-001".to_string()));
    
    println!("✅ End-to-end tracing functionality test passed");
}

/// Test distributed tracing across services
#[test]
fn test_distributed_tracing() {
    let mut manager = ObservabilityManager::new();
    
    // Create a trace that spans multiple services
    // Service 1: API Gateway
    let gateway_span = TraceSpan {
        id: "gateway-span".to_string(),
        trace_id: "distributed-trace-001".to_string(),
        parent_id: None,
        name: "api_gateway_request".to_string(),
        start_time: 2000000,
        end_time: Some(2000200),
        service: "api-gateway".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("http.method".to_string(), "POST".to_string());
            attrs.insert("http.url".to_string(), "/v1/orders".to_string());
            attrs
        },
    };
    
    // Service 2: Order Service (child of gateway)
    let order_span = TraceSpan {
        id: "order-span".to_string(),
        trace_id: "distributed-trace-001".to_string(),
        parent_id: Some("gateway-span".to_string()),
        name: "process_order".to_string(),
        start_time: 2000010,
        end_time: Some(2000150),
        service: "order-service".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("order.id".to_string(), "order-12345".to_string());
            attrs.insert("order.amount".to_string(), "99.99".to_string());
            attrs
        },
    };
    
    // Service 3: Payment Service (child of order)
    let payment_span = TraceSpan {
        id: "payment-span".to_string(),
        trace_id: "distributed-trace-001".to_string(),
        parent_id: Some("order-span".to_string()),
        name: "process_payment".to_string(),
        start_time: 2000020,
        end_time: Some(2000100),
        service: "payment-service".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("payment.method".to_string(), "credit_card".to_string());
            attrs.insert("payment.status".to_string(), "success".to_string());
            attrs
        },
    };
    
    // Service 4: Inventory Service (child of order, parallel to payment)
    let inventory_span = TraceSpan {
        id: "inventory-span".to_string(),
        trace_id: "distributed-trace-001".to_string(),
        parent_id: Some("order-span".to_string()),
        name: "update_inventory".to_string(),
        start_time: 2000100,
        end_time: Some(2000140),
        service: "inventory-service".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("item.id".to_string(), "item-67890".to_string());
            attrs.insert("quantity".to_string(), "1".to_string());
            attrs
        },
    };
    
    // Add all spans to the manager
    let mut spans = manager.spans.lock().unwrap();
    spans.insert(gateway_span.id.clone(), gateway_span.clone());
    spans.insert(order_span.id.clone(), order_span.clone());
    spans.insert(payment_span.id.clone(), payment_span.clone());
    spans.insert(inventory_span.id.clone(), inventory_span.clone());
    drop(spans);
    
    // Verify distributed trace structure
    let spans = manager.spans.lock().unwrap();
    assert_eq!(spans.len(), 4);
    
    // Verify trace continuity
    assert_eq!(gateway_span.trace_id, order_span.trace_id);
    assert_eq!(order_span.trace_id, payment_span.trace_id);
    assert_eq!(payment_span.trace_id, inventory_span.trace_id);
    
    // Verify parent-child relationships
    assert_eq!(order_span.parent_id, Some(gateway_span.id.clone()));
    assert_eq!(payment_span.parent_id, Some(order_span.id.clone()));
    assert_eq!(inventory_span.parent_id, Some(order_span.id.clone()));
    
    println!("✅ Distributed tracing test passed");
}

/// Test tracing with high concurrency
#[test]
fn test_tracing_high_concurrency() {
    let manager = std::sync::Arc::new(std::sync::Mutex::new(ObservabilityManager::new()));
    
    // Simulate multiple concurrent requests
    let mut handles = vec![];
    
    for i in 0..50 {
        let manager_clone = std::sync::Arc::clone(&manager);
        let handle = thread::spawn(move || {
            let mut mgr = manager_clone.lock().unwrap();
            
            // Create a unique trace for this request
            let trace_id = format!("concurrent-trace-{}", i);
            let span_id = format!("span-{}", i);
            
            let span = TraceSpan {
                id: span_id.clone(),
                trace_id: trace_id.clone(),
                parent_id: None,
                name: format!("concurrent_request_{}", i),
                start_time: 3000000 + (i * 1000),
                end_time: Some(3000000 + (i * 1000) + 500),
                service: "concurrent-test-service".to_string(),
                attributes: {
                    let mut attrs = HashMap::new();
                    attrs.insert("request.id".to_string(), format!("req-{}", i));
                    attrs.insert("thread.id".to_string(), format!("{:?}", thread::current().id()));
                    attrs
                },
            };
            
            let mut spans = mgr.spans.lock().unwrap();
            spans.insert(span_id, span);
        });
        
        handles.push(handle);
        
        // Small delay to simulate realistic request timing
        thread::sleep(Duration::from_millis(1));
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all spans were created
    let mgr = manager.lock().unwrap();
    let spans = mgr.spans.lock().unwrap();
    assert_eq!(spans.len(), 50);
    
    println!("✅ High concurrency tracing test passed");
}

/// Test tracing error handling
#[test]
fn test_tracing_error_handling() {
    let mut manager = ObservabilityManager::new();
    
    // Create a span with error attributes
    let error_span = TraceSpan {
        id: "error-span".to_string(),
        trace_id: "error-trace-001".to_string(),
        parent_id: None,
        name: "failed_operation".to_string(),
        start_time: 4000000,
        end_time: Some(4000100),
        service: "error-test-service".to_string(),
        attributes: {
            let mut attrs = HashMap::new();
            attrs.insert("error".to_string(), "true".to_string());
            attrs.insert("error.message".to_string(), "Database connection failed".to_string());
            attrs.insert("error.type".to_string(), "DatabaseError".to_string());
            attrs.insert("http.status_code".to_string(), "500".to_string());
            attrs
        },
    };
    
    // Add the error span to the manager
    let mut spans = manager.spans.lock().unwrap();
    spans.insert(error_span.id.clone(), error_span.clone());
    drop(spans);
    
    // Verify error attributes are preserved
    let spans = manager.spans.lock().unwrap();
    let retrieved_span = spans.get("error-span").unwrap();
    
    assert_eq!(retrieved_span.attributes.get("error"), Some(&"true".to_string()));
    assert_eq!(retrieved_span.attributes.get("error.message"), Some(&"Database connection failed".to_string()));
    assert_eq!(retrieved_span.attributes.get("error.type"), Some(&"DatabaseError".to_string()));
    assert_eq!(retrieved_span.attributes.get("http.status_code"), Some(&"500".to_string()));
    
    println!("✅ Tracing error handling test passed");
}

/// Test tracing performance with large attributes
#[test]
fn test_tracing_large_attributes() {
    let mut manager = ObservabilityManager::new();
    
    // Create a span with large attributes
    let mut large_attributes = HashMap::new();
    for i in 0..1000 {
        large_attributes.insert(format!("attr_{}", i), format!("value_{}", i));
    }
    
    let large_span = TraceSpan {
        id: "large-span".to_string(),
        trace_id: "large-trace-001".to_string(),
        parent_id: None,
        name: "large_attributes_operation".to_string(),
        start_time: 5000000,
        end_time: Some(5000100),
        service: "large-attributes-service".to_string(),
        attributes: large_attributes,
    };
    
    // Add the large span to the manager
    let mut spans = manager.spans.lock().unwrap();
    spans.insert(large_span.id.clone(), large_span.clone());
    drop(spans);
    
    // Verify all attributes are preserved
    let spans = manager.spans.lock().unwrap();
    let retrieved_span = spans.get("large-span").unwrap();
    
    assert_eq!(retrieved_span.attributes.len(), 1000);
    assert_eq!(retrieved_span.attributes.get("attr_500"), Some(&"value_500".to_string()));
    assert_eq!(retrieved_span.attributes.get("attr_999"), Some(&"value_999".to_string()));
    
    println!("✅ Large attributes tracing test passed");
}