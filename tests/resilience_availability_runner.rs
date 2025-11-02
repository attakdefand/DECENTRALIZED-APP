//! Resilience and Availability Simulation Test Runner
//!
//! This is a simple test runner for the resilience and availability simulation tests.

mod resilience_availability_simulation;

fn main() {
    println!("Running Resilience and Availability Simulation Tests");
    
    // Run all simulation tests
    resilience_availability_simulation::test_realistic_resilience_availability_scenario();
    resilience_availability_simulation::test_resilience_availability_under_stress();
    resilience_availability_simulation::test_resilience_availability_error_scenarios();
    
    println!("All Resilience and Availability Simulation Tests Passed!");
}