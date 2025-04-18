# VIN-UI

A modern CLI tool for super seamlessly adding UI components to your Next.js projects.

## 🚀 Quick Start Guide

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)
- A Next.js project where you want to add components

### Installation

1. Clone the repository
```bash
git clone https://github.com/vin-dKR/vin-ui-cli
cd vin-ui
```

2. Build and install the CLI
```bash
cargo build --release
cargo install --path .
```

3. Verify installation
```bash
vin-ui --help
```

You should see the VIN-UI help message with available commands.

## 📋 Usage

### Initialize Your Project

Before adding components, initialize your Next.js project:

```bash
cd your-nextjs-project
vin-ui init
```

This will create the necessary directory structure:
- `components/ui/` - Where UI components will be installed
- `lib/utils.ts` - For utility functions that components may need

### List Available Components

To see what components are available to install:

```bash
vin-ui list
```

This will show all components in the templates directory, along with their dependencies and required utilities.

### Add Components

To add a component to your project:

```bash
vin-ui add Button
```

The CLI will:
1. Copy the component file to your `components/ui/` directory
2. Offer to install any required dependencies
3. Add any necessary utility functions to `lib/utils.ts`

## 🧩 Adding Custom Components to Templates

You can expand the template library with your own components:

1. Create your component file in the `templates/` directory:
```
templates/MyComponent.tsx
```

2. (Optional) Create a configuration file with the same name:
```
templates/MyComponent.json
```

The configuration file should follow this format:

```json
{
    "name": "MyComponent",
    "dependencies": [
        "framer-motion",
        "other-dependency"
    ],
    "utils": [
        "cn",
        "other-utility"
    ]
}
```

3. Add any utility functions to `templates/utils/`:
```
templates/utils/my-utility.ts
```

## 💡 Tips for Better Usage

- **Create component families**: Group related components together with consistent naming (e.g., `Form`, `FormInput`, `FormLabel`)
- **Document component dependencies**: Always create configuration files with proper dependencies
- **Use consistent styling**: Ensure your components follow the same styling approach (e.g., Tailwind CSS)
- **Test components before adding**: Make sure your template components work properly before adding them to the templates directory

## 🔧 Troubleshooting

### Command not found

If `vin-ui` command is not found after installation:

1. Make sure Cargo's bin directory is in your PATH:
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

2. Try reinstalling:
```bash
cargo install --force --path .
```

### Component not found

If a component can't be found when using `vin-ui add`:

1. Check that the component exists in the templates directory
2. Ensure the filename matches exactly (case-sensitive)
3. Run `vin-ui list` to see available components

### Dependency installation fails

If dependency installation fails:

1. Make sure you have a working internet connection
2. Try installing the dependency manually: `npm install <dependency-name>` or `yarn add <dependency-name>`
3. Check if the dependency exists in the npm registry

## 🤝 Contributing

Contributions are welcome! Here's how you can contribute:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin feature/my-feature`
5. Submit a pull request

## 📃 License

This project is licensed under the MIT License - see the LICENSE file for details.
