# Docker Extension for Zed

A **Docker commands palette** for the [Zed editor](https://zed.dev/). This extension provides commands to interact with docker directly from the editor commands palette

## Requirements

- Zed editor v0.160.0 or later
- Docker CLI

## Features Roadmap (v1.0.0) 
- 🔄 **Containers**
  - Start containers
  - Stop containers
  - Restart containers
  - Inspect containers
  - Delete containers
- ❌ **Images**:
  - List images
  - Pull Images
  - Delete images
- ❌ **Registries**:
  - List registries
  - Add registry
  - Delete registry

## Installation

### From Zed Extensions (Coming Soon)
1. Open Zed
2. Press `cmd+shift+p` (Mac) or `ctrl+shift+p` (Windows/Linux)
3. Search for "zed: extensions"
4. Search for "Docker"
5. Click Install

### Development Installation
1. Clone this repository
2. In Zed, open the extensions view (`cmd+shift+p` → "zed: extensions")
3. Click "Install Dev Extension"
4. Select the `zed-docker` directory

## Usage

This extension is designed for **provide commands to interact with docker directly from the editor commands palette**.

### Good Use Cases

- ✅ Quickly start/stop/restart/delete containers
- ✅ Quickly manage local images

## Development Roadmap

### 🔄 Phase 1: Containers commands (In Development)
- Start/Stop/Restart/Delete containers

### 🔮 Phase 2: Images commands
- List/Pull/Delete images

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

- Built with the [Zed Extension API](https://github.com/zed-industries/zed)