# Web UI Deployment Guide

## Overview

This document describes how to deploy the Web UI component of the DECENTRALIZED-APP project. The web-ui is a WebAssembly application built with Rust and the Yew framework.

## Prerequisites

- Rust toolchain
- wasm-pack
- Node.js (for development server)
- Docker (optional, for containerized deployment)

## Build Process

### Development Build

```bash
# Navigate to the web-ui directory
cd web-ui

# Build the WebAssembly package
wasm-pack build --target web
```

### Production Build

```bash
# Navigate to the web-ui directory
cd web-ui

# Build the WebAssembly package for production
wasm-pack build --target web --release
```

## Deployment Options

### 1. Static File Hosting

The simplest deployment method is to serve the files statically:

1. Build the WebAssembly package
2. Copy the contents of the `pkg/` directory to your web server
3. Serve the `index.html` file and associated assets

### 2. GitHub Pages

The project includes a GitHub Actions workflow for automatic deployment to GitHub Pages:

1. Push changes to the `main` branch
2. The workflow in `.github/workflows/web-ui.yml` will automatically build and deploy
3. The site will be available at `https://<username>.github.io/<repository>/`

### 3. Docker Deployment

Create a Docker image for the web-ui:

```dockerfile
FROM nginx:alpine

COPY web-ui/pkg /usr/share/nginx/html/pkg
COPY web-ui/index.html /usr/share/nginx/html/
COPY web-ui/styles.css /usr/share/nginx/html/
COPY web-ui/simple-server.py /usr/share/nginx/html/

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
```

Build and run the Docker container:

```bash
docker build -t dex-web-ui .
docker run -p 8080:80 dex-web-ui
```

### 4. Integration with Existing Infrastructure

The web-ui can be integrated with the existing Docker Compose setup:

1. Add a web service to `infra/docker-compose.yml`
2. Mount the web-ui assets as volumes
3. Configure nginx to serve the static files

## Environment Configuration

### API Endpoint Configuration

The web-ui connects to the API service. The base URL is configured in `src/services/api.rs`:

```rust
pub fn create_client() -> ApiClient {
    // In a real app, this would be configurable
    ApiClient::new("http://localhost:3000/api/v1".to_string())
}
```

For production deployment, this should be updated to point to the actual API service.

### Environment Variables

Currently, the web-ui does not use environment variables, but this could be added for configuration:

```rust
use web_sys::window;

fn get_api_base_url() -> String {
    // Try to get from environment variable or use default
    window()
        .and_then(|w| w.location().hostname().ok())
        .map(|hostname| format!("http://{}:3000/api/v1", hostname))
        .unwrap_or_else(|| "http://localhost:3000/api/v1".to_string())
}
```

## Monitoring and Observability

### Client-Side Error Tracking

Implement error tracking in the web-ui:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: &str);
}

fn log_error(msg: &str) {
    error(&format!("Web UI Error: {}", msg));
}
```

### Performance Monitoring

Add performance monitoring for critical operations:

```rust
use web_sys::console;
use wasm_bindgen::JsValue;

fn log_performance(operation: &str, duration: f64) {
    console::log_2(
        &JsValue::from_str("Performance"),
        &JsValue::from_str(&format!("{} took {}ms", operation, duration))
    );
}
```

## Security Considerations

### Content Security Policy

Implement a strict Content Security Policy in `index.html`:

```html
<meta http-equiv="Content-Security-Policy" 
      content="default-src 'self'; 
               script-src 'self' 'unsafe-eval'; 
               style-src 'self' 'unsafe-inline'; 
               img-src 'self' data:; 
               connect-src 'self' http://localhost:3000;">
```

### Input Validation

All user inputs should be validated before being sent to the API:

```rust
fn validate_token_address(address: &str) -> bool {
    // Simple validation - in a real app, use proper validation
    address.len() == 42 && address.starts_with("0x")
}
```

## Scaling Considerations

### Caching Strategy

Implement caching for static assets:

1. Set appropriate cache headers for WebAssembly files
2. Use service workers for offline support
3. Implement client-side caching for API responses

### Load Balancing

For high-traffic deployments:

1. Deploy multiple instances behind a load balancer
2. Use a CDN for static assets
3. Implement API rate limiting

## Troubleshooting

### Common Issues

1. **WebAssembly module not loading**
   - Check that all files in the `pkg/` directory are deployed
   - Verify CORS settings if serving from a different domain

2. **API connection failures**
   - Ensure the API service is running and accessible
   - Check the base URL configuration

3. **Rendering issues**
   - Verify that Tailwind CSS is properly loaded
   - Check browser compatibility

### Debugging

Enable debug logging in development:

```rust
#[cfg(debug_assertions)]
use web_sys::console;

#[cfg(debug_assertions)]
fn debug_log(msg: &str) {
    console::log_1(&JsValue::from_str(&format!("DEBUG: {}", msg)));
}
```

## Maintenance

### Updating Dependencies

Regularly update dependencies:

```bash
cd web-ui
cargo update
```

### Performance Optimization

1. Monitor bundle sizes
2. Optimize WebAssembly compilation
3. Implement code splitting for large applications

### Backward Compatibility

When making changes:
1. Ensure API compatibility
2. Maintain UI consistency
3. Test with different browsers