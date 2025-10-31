# 🚀 Decentralized Exchange - Deployment Guide

## Quick Start with Docker Compose

```bash
# 1. Clone and navigate
cd DECENTRALIZED-APP-main

# 2. Copy environment template
cp .env.example .env

# 3. Edit .env with secure passwords
# IMPORTANT: Change all passwords in production!

# 4. Start all services
docker-compose up -d

# 5. Check service health
docker-compose ps
```

## Services Overview

| Service | Port | Purpose | Health Check |
|---------|------|---------|--------------|
| **PostgreSQL** | 5432 | Database | `pg_isready` |
| **Redis** | 6379 | Cache/Rate Limit | `redis-cli ping` |
| **API Service** | 3000 | REST + WebSocket | `/health` |
| **Indexer** | - | Blockchain sync | - |
| **MEV Monitor** | - | MEV detection | - |
| **IPFS** | 4001, 5001, 8080 | Storage | - |
| **Keepers** | - | Automation | - |
| **Prometheus** | 9090 | Metrics | - |
| **Grafana** | 3001 | Dashboards | - |
| **Nginx** | 80, 443 | Reverse proxy | - |

## Testing the Stack

### 1. Test API Health
```bash
curl http://localhost:3000/health
# Expected: "OK"
```

### 2. Test REST Endpoints
```bash
# Get pools
curl http://localhost:3000/api/v1/pools

# Get markets
curl http://localhost:3000/api/v1/markets

# Get orders
curl http://localhost:3000/api/v1/orders

# Create order
curl -X POST http://localhost:3000/api/v1/orders \
  -H "Content-Type: application/json" \
  -d '{
    "pair": "ETH/USDC",
    "side": "buy",
    "price": 2500.0,
    "amount": 1.0
  }'
```

### 3. Test WebSocket Connection
```javascript
// Open browser console at http://localhost:8080
const ws = new WebSocket('ws://localhost:3000/ws');

ws.onopen = () => {
  console.log('Connected!');
  
  // Subscribe to channels
  ws.send(JSON.stringify({
    type: 'subscribe',
    channels: ['markets', 'orders', 'pools']
  }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Received:', data);
};

// Send test message
ws.send(JSON.stringify({
  type: 'pong',
  timestamp: Date.now()
}));
```

### 4. Test Database
```bash
# Connect to PostgreSQL
docker-compose exec postgres psql -U dex_user -d decentralized_exchange

# Run queries
SELECT * FROM pools;
SELECT * FROM markets;
SELECT * FROM orders;
```

### 5. View Metrics
```bash
# Open in browser
http://localhost:9090  # Prometheus
http://localhost:3001  # Grafana (admin/admin_change_in_production)
```

## Production Deployment

### 1. Security Hardening

**Change all passwords in .env:**
```bash
# Generate strong passwords
DB_PASSWORD=$(openssl rand -base64 32)
REDIS_PASSWORD=$(openssl rand -base64 32)
JWT_SECRET=$(openssl rand -base64 48)
```

**Enable SSL/TLS:**
```yaml
# In docker-compose.yml, add to nginx:
volumes:
  - ./certs/fullchain.pem:/etc/nginx/ssl/fullchain.pem:ro
  - ./certs/privkey.pem:/etc/nginx/ssl/privkey.pem:ro
```

**Firewall rules:**
```bash
# Only expose necessary ports
ufw allow 80/tcp    # HTTP
ufw allow 443/tcp   # HTTPS
ufw deny 5432/tcp   # PostgreSQL (internal only)
ufw deny 6379/tcp   # Redis (internal only)
```

### 2. Performance Tuning

**PostgreSQL:**
```bash
# In docker-compose.yml, add to postgres:
environment:
  POSTGRES_SHARED_BUFFERS: 256MB
  POSTGRES_EFFECTIVE_CACHE_SIZE: 1GB
  POSTGRES_MAX_CONNECTIONS: 100
```

**Redis:**
```bash
# In docker-compose.yml, add to redis:
command: redis-server --maxmemory 512mb --maxmemory-policy allkeys-lru
```

### 3. Backup Strategy

**Database backups:**
```bash
# Daily backup script
docker-compose exec postgres pg_dump -U dex_user decentralized_exchange > backup-$(date +%Y%m%d).sql

# Restore
docker-compose exec -T postgres psql -U dex_user decentralized_exchange < backup.sql
```

**Volume backups:**
```bash
# Backup all volumes
docker run --rm -v dex-postgres:/data -v $(pwd):/backup alpine tar czf /backup/postgres-data.tar.gz /data
```

### 4. Monitoring Setup

**Prometheus targets:**
```yaml
# infra/prometheus.yml
scrape_configs:
  - job_name: 'api-service'
    static_configs:
      - targets: ['api-service:3000']
  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres-exporter:9187']
  - job_name: 'redis'
    static_configs:
      - targets: ['redis-exporter:9121']
```

**Alert rules:**
```yaml
# infra/alerts.yml
groups:
  - name: api
    rules:
      - alert: HighErrorRate
        expr: rate(http_request_errors_total[5m]) > 0.05
        for: 5m
        annotations:
          summary: "High error rate detected"
```

## Troubleshooting

### Service won't start
```bash
# Check logs
docker-compose logs api-service
docker-compose logs postgres

# Check resource usage
docker stats

# Restart specific service
docker-compose restart api-service
```

### Database connection fails
```bash
# Verify database is ready
docker-compose exec postgres pg_isready -U dex_user

# Check connection string
echo $DATABASE_URL

# Test connection
docker-compose exec api-service /app/api
```

### WebSocket disconnects
```bash
# Check nginx config for WebSocket upgrade
# Ensure these headers are set:
proxy_http_version 1.1;
proxy_set_header Upgrade $http_upgrade;
proxy_set_header Connection "upgrade";
```

### High memory usage
```bash
# Check container memory limits
docker-compose exec api-service free -h

# Add memory limits to docker-compose.yml:
deploy:
  resources:
    limits:
      memory: 512M
```

## Development Workflow

### Run without Docker
```bash
# Start dependencies
docker-compose up postgres redis

# Set environment
export DATABASE_URL="postgresql://dex_user:password@localhost:5432/decentralized_exchange"
export REDIS_URL="redis://:password@localhost:6379"

# Run API service
cd services/api-rs
cargo run

# Run tests
cargo test
```

### Hot reload for development
```bash
# Install cargo-watch
cargo install cargo-watch

# Auto-rebuild on changes
cd services/api-rs
cargo watch -x run
```

## Scaling

### Horizontal Scaling
```bash
# Scale API service to 3 instances
docker-compose up -d --scale api-service=3

# Add load balancer
# Configure nginx to round-robin across instances
```

### Database Read Replicas
```yaml
# Add to docker-compose.yml
postgres-replica:
  image: postgres:15-alpine
  environment:
    POSTGRES_MASTER_SERVICE_HOST: postgres
  command: |
    bash -c "pg_basebackup -h postgres -D /var/lib/postgresql/data -U replication -v -P && 
    postgres -c hot_standby=on"
```

## Security Checklist

- [ ] All passwords changed from defaults
- [ ] SSL/TLS certificates installed
- [ ] Firewall rules configured
- [ ] Database backups automated
- [ ] Monitoring alerts configured
- [ ] Rate limiting enabled
- [ ] CORS properly configured
- [ ] Non-root containers running
- [ ] Secrets in environment variables (not in code)
- [ ] Health checks working
- [ ] Logs centralized
- [ ] Dependency updates scheduled

## Support

For issues, check:
1. `docker-compose logs <service>` - Service logs
2. `docker-compose ps` - Service status
3. `docker stats` - Resource usage
4. GitHub Issues - Community support
