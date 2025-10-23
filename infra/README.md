# Infrastructure

This directory contains the infrastructure components for the decentralized application.

## Components

1. **PostgreSQL** - Main database for OLTP operations
2. **ClickHouse** - Analytics database for OLAP operations
3. **Redis** - Caching and queue management
4. **NATS** - Event streaming and messaging
5. **IPFS** - Content storage
6. **Prometheus** - Metrics collection
7. **Grafana** - Dashboard and visualization

## Getting Started

To start all infrastructure components:

```bash
docker-compose up -d
```

To stop all infrastructure components:

```bash
docker-compose down
```

## Services

- PostgreSQL: localhost:5432
- ClickHouse: localhost:8123
- Redis: localhost:6379
- NATS: localhost:4222
- IPFS: localhost:5001
- Prometheus: localhost:9090
- Grafana: localhost:3001