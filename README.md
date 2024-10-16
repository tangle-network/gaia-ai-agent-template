# Gaia AI Agent AVS Blueprint

## Overview

The Gaia AI Agent AVS Blueprint is a templated Autonomous Validation Service (AVS) Blueprint for managing and interacting with Gaia AI Nodes. This blueprint leverages the Tangle network for decentralized deployment and management, with potential integration into the EigenLayer ecosystem.

## Key Features

1. **Gaia Node Management**
   - Run, stop, upgrade, and configure Gaia nodes
   - Onchain operator management via Tangle network

2. **AI Interaction Services**
   - Chat with AI models
   - Analyze images
   - Create images
   - Edit images

3. **Flexible Model Selection**
   - Support for various AI models compatible with different tasks

4. **Decentralized Deployment**
   - Instance across registered Tangle operators
   - Potential EigenLayer AVS integration

## Architecture

The blueprint consists of two main components:

1. **Tangle Blueprint**: Manages Gaia node operations and service instantiation.
2. **HTTP Server**: Exposes AI interaction endpoints for users.

## Usage

### For Developers

- Create new extensions and add new functions.
- Integrate realtime websocket service.
- Add incentives and payments.
- Add api keys for managing auth/access-control.

### For Operators

1. Register for Gaia Tangle Blueprints
2. Instance the service on Tangle operators.
3. Manage Gaia nodes using onchain transactions.

### For Users

Interact with the AI services via HTTP endpoints:
- `/chat`: Chat with the AI model
- `/analyze_image`: Analyze images
- `/create_image`: Generate images
- `/edit_image`: Edit existing images

## Development

Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)
- [Forge](https://getfoundry.sh)
- [Tangle](https://github.com/tangle-network/tangle?tab=readme-ov-file#-getting-started-)
- **`cargo install cargo-tangle`** or [Tangle CLI Download Link](https://github.com/webb-tools/gadget/releases/download/cargo-tangle-v0.1.2/cargo-tangle-installer.sh)

Build the project:
```bash
cargo build
```

## 📜 License

This project is licensed under the unlicense License. See the [LICENSE](./LICENSE) file for more details.