-- Create pools table
CREATE TABLE IF NOT EXISTS pools (
    id VARCHAR(255) PRIMARY KEY,
    token_a_symbol VARCHAR(10) NOT NULL,
    token_a_address VARCHAR(66) NOT NULL,
    token_b_symbol VARCHAR(10) NOT NULL,
    token_b_address VARCHAR(66) NOT NULL,
    liquidity VARCHAR(50) NOT NULL,
    volume_24h VARCHAR(50) NOT NULL DEFAULT '0',
    apr VARCHAR(10) NOT NULL DEFAULT '0',
    fee_tier VARCHAR(10) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create orders table
CREATE TABLE IF NOT EXISTS orders (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    pair VARCHAR(20) NOT NULL,
    side VARCHAR(10) NOT NULL CHECK (side IN ('buy', 'sell')),
    price VARCHAR(50) NOT NULL,
    amount VARCHAR(50) NOT NULL,
    filled VARCHAR(50) NOT NULL DEFAULT '0',
    status VARCHAR(20) NOT NULL CHECK (status IN ('open', 'filled', 'cancelled', 'partial')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create markets table
CREATE TABLE IF NOT EXISTS markets (
    pair VARCHAR(20) PRIMARY KEY,
    price VARCHAR(50) NOT NULL,
    change_24h VARCHAR(10) NOT NULL,
    volume_24h VARCHAR(50) NOT NULL,
    high_24h VARCHAR(50) NOT NULL,
    low_24h VARCHAR(50) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_pools_volume ON pools(volume_24h DESC);
CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
CREATE INDEX IF NOT EXISTS idx_orders_created_at ON orders(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_markets_volume ON markets(volume_24h DESC);

-- Insert sample data for pools
INSERT INTO pools (id, token_a_symbol, token_a_address, token_b_symbol, token_b_address, liquidity, volume_24h, apr, fee_tier)
VALUES 
    ('pool-eth-usdc-001', 'ETH', '0x0000000000000000000000000000000000000001', 'USDC', '0x0000000000000000000000000000000000000002', '1250000.75', '45000.30', '12.5', '0.3'),
    ('pool-btc-usdc-001', 'BTC', '0x0000000000000000000000000000000000000003', 'USDC', '0x0000000000000000000000000000000000000002', '2500000.00', '87000.45', '8.75', '0.3')
ON CONFLICT (id) DO NOTHING;

-- Insert sample data for markets
INSERT INTO markets (pair, price, change_24h, volume_24h, high_24h, low_24h)
VALUES 
    ('ETH/USDC', '2530.0', '2.5', '45000000.0', '2550.0', '2480.0'),
    ('BTC/USDC', '45300.0', '-1.2', '87000000.0', '46000.0', '44800.0')
ON CONFLICT (pair) DO UPDATE SET
    price = EXCLUDED.price,
    change_24h = EXCLUDED.change_24h,
    volume_24h = EXCLUDED.volume_24h,
    high_24h = EXCLUDED.high_24h,
    low_24h = EXCLUDED.low_24h,
    updated_at = NOW();
