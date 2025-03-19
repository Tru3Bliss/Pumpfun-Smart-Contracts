# Getting Started with Pumpfun Smart Contract

## Prerequisites

Before you begin, ensure you have the following installed:

- Node.js (v16 or higher)
- Yarn
- Solana CLI
- Anchor Framework
- Git

## Installation

1. Clone the repository:
```bash
git clone https://github.com/Tru3Bliss/Pumpfun-Smart-Contract-All.git
cd Pumpfun-Smart-Contract-All
```

2. Install dependencies:
```bash
yarn install
```

3. Build the project:
```bash
anchor build
```

## Development Setup

1. Configure Solana CLI:
```bash
solana config set --url devnet
```

2. Create a new wallet:
```bash
solana-keygen new
```

3. Get some devnet SOL:
```bash
solana airdrop 2
```

## Testing

Run the test suite:
```bash
anchor test
```

## Deployment

Deploy to devnet:
```bash
anchor deploy
```

## Next Steps

- Read the [Smart Contract Overview](features/smart-contract.md)
- Learn about [DEX Integration](features/dex-integration.md)
- Explore [Token Management](features/token-management.md)
- Understand [Virtual Pools](features/virtual-pools.md)

## Need Help?

- Check our [Support Policy](.github/SUPPORT.md)
- Join our [Telegram](https://t.me/Tru3B1iss)
- Follow us on [Twitter](https://x.com/XTruebliss)
- Connect on [Discord](https://discord.com/users/1274339638668038187) 