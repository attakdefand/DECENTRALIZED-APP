// Cedar policy for the decentralized application

// Allow principals to read pools
permit (
    principal,
    action == Action::"ReadPools",
    resource == Pool::"all"
);

// Allow principals to read orders
permit (
    principal,
    action == Action::"ReadOrders",
    resource == Order::"all"
);

// Allow principals to read markets
permit (
    principal,
    action == Action::"ReadMarkets",
    resource == Market::"all"
);

// Allow authenticated users to create orders
permit (
    principal is User,
    action == Action::"CreateOrder",
    resource is Order
) when {
    principal.isAuthenticated
};

// Deny unauthenticated users from creating orders
forbid (
    principal is User,
    action == Action::"CreateOrder",
    resource is Order
) when {
    not principal.isAuthenticated
};