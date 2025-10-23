# Web UI for Decentralized Exchange

This is the WebAssembly-based frontend for the Decentralized Exchange application, built with Rust and the Yew framework.

## Role and Function in the DECENTRALIZED-APP Project

The web-ui serves as the primary user interface for interacting with the decentralized exchange. It provides a responsive, modern web application that allows users to:

1. **View Liquidity Pools** - Browse available trading pairs and their liquidity
2. **Monitor Orders** - Track active and historical trades
3. **Analyze Markets** - View price charts and market data
4. **Trade Tokens** - Execute swaps and other trading operations

The web-ui communicates with the off-chain services through RESTful APIs:
- **api-rs** - Provides data endpoints for pools, orders, and markets
- **indexer-rs** - Supplies indexed blockchain event data
- **keepers-rs** - Offers status information for automated keeper operations

## Architecture

The web-ui follows a component-based architecture using the Yew framework:

```
src/
├── components/     # Reusable UI components
├── pages/          # Page-level components
├── services/       # API service layer
├── router.rs       # Application routing
└── lib.rs          # Main application entry point
```

## Key Features

- **Responsive Design** - Works on desktop and mobile devices
- **Real-time Data** - Displays live market information
- **Type Safety** - Full Rust type safety for API interactions
- **WebAssembly Performance** - Near-native performance in the browser
- **Modular Structure** - Easy to extend and maintain

## Development

### Prerequisites

- Rust and Cargo
- wasm-pack
- Node.js (for development server)

### Building

```bash
# Build the WebAssembly package
wasm-pack build --target web

# Start development server
python simple-server.py
```

### Testing

```bash
# Run unit tests
wasm-pack test --headless --firefox
```

## Deployment

The web-ui can be deployed as a static website. The WebAssembly module is compiled to a `pkg/` directory which contains all necessary files for deployment.

## Integration with Other Services

The web-ui integrates with other services in the DECENTRALIZED-APP project:

1. **api-rs** - Fetches pool, order, and market data
2. **indexer-rs** - Retrieves indexed blockchain events
3. **keepers-rs** - Shows keeper status and operations
4. **mev-monitor** - Displays MEV-related information
5. **ipfs-rs** - Accesses decentralized storage when needed

This integration allows users to have a complete view of the decentralized exchange operations through a single, unified interface.

## Technologies Used

- **Rust**: Core programming language
- **WebAssembly (WASM)**: Compilation target for running Rust in the browser
- **Yew**: Rust framework for building web applications
- **wasm-pack**: Tool for building WebAssembly packages
- **Tailwind CSS**: Utility-first CSS framework (classes used in HTML)

## Getting Started

### Prerequisites

1. Rust and Cargo installed
2. wasm-pack installed (`cargo install wasm-pack`)
3. Node.js and npm (for development server)

### Building

#### Using PowerShell (Windows)

```powershell
.\build.ps1
```

#### Using Bash (Unix/Linux/macOS)

```bash
./build.sh
```

This will compile the Rust code to WebAssembly and generate the necessary JavaScript bindings in the `pkg/` directory.

### Development Server

To run a development server with hot reloading:

```powershell
.\dev-server.ps1
```

This will start a local server on `http://localhost:8080`.

## Architecture

### Components

The UI is built using a component-based architecture:

- **Navigation**: Main navigation bar
- **PoolCard**: Displays information about a liquidity pool
- **OrderTable**: Shows a table of orders
- **MarketChart**: Displays price charts using SVG

### Pages

The application has several main pages:

- **Home**: Landing page with overview
- **Pools**: Liquidity pools management
- **Orders**: Order management
- **Markets**: Market data and charts
- **NotFound**: 404 error page

### Services

API services handle communication with the backend:

- **ApiClient**: Base HTTP client
- **PoolsService**: Pool-related API calls
- **OrdersService**: Order-related API calls
- **MarketsService**: Market data API calls

## Styling

The application uses Tailwind CSS utility classes directly in the HTML/RSX code. The `styles.css` file contains some additional custom styles and imports the Inter font.

## Deployment

To deploy the web UI:

1. Build the WebAssembly package
2. Serve the `index.html` file and `pkg/` directory with a web server
3. Ensure CORS is configured correctly if the API is on a different domain

## Development

### Adding New Components

1. Create a new file in the `src/components/` directory
2. Add the component to `src/components/mod.rs`
3. Import and use the component in pages

### Adding New Pages

1. Create a new file in the `src/pages/` directory
2. Add the page to `src/pages/mod.rs`
3. Add a new route in `src/router.rs`
4. Add navigation link in the navigation component

### Adding New Services

1. Create a new file in the `src/services/` directory
2. Add the service to `src/services/mod.rs`
3. Use the service in components or pages

## Browser Support

The WebAssembly web UI requires a modern browser with WebAssembly support:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run the build script to ensure everything works
5. Submit a pull request