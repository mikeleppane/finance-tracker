<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# 💰 Finance Tracker - Built with Leptos & Rust

A modern, full-stack personal finance tracking application built with **Leptos**, **Rust**, and **Azure Cosmos DB**. This project demonstrates clean architecture, server-side rendering, and type-safe full-stack development.

> 📝 **Blog Series**: This repository accompanies the **"Track Your Finances with Leptos & Rust"** blog series published on [Medium](https://medium.com/stackademic/an-in-depth-exploration-of-the-internals-of-robot-framework-part-i-7f6f0795ac92). Follow along as we build a complete finance application from scratch!

## ✨ Features

- 🔐 **JWT Authentication** with refresh token support
- 📊 **Dashboard** with financial overview and insights  
- 💳 **Transaction Management** - add, edit, categorize expenses/income
- 🎯 **Budget Planning** with tracking and alerts
- 📈 **Financial Reports** and analytics
- 🎨 **Responsive Design** with TailwindCSS
- ⚡ **Server-Side Rendering** for optimal performance
- 🛡️ **Type Safety** across the entire stack
- ☁️ **Azure Cosmos DB** integration
- 🧪 **End-to-End Testing** with Playwright

## 🏗️ Architecture

This project follows **Clean Architecture** principles with clear separation of concerns:

```
src/
├── 🏛️ domain/          # Business logic & entities
├── 🔧 application/     # Use cases & services  
├── 🌐 infrastructure/  # External integrations
├── 🎨 components/      # UI components
└── 📄 presentation/    # Pages & routing
```

## 🚀 Quick Start

### Prerequisites

- **Rust** (nightly toolchain)
- **Node.js** (for TailwindCSS)
- **Azure Cosmos DB** account (or Azure Cosmos DB Emulator)

### Installation

1. **Install cargo-leptos**:
   ```bash
   cargo install cargo-leptos --locked
   ```

2. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/finance-tracker
   cd finance-tracker
   ```

3. **Install dependencies**:
   ```bash
   # Rust dependencies are automatically installed
   npm install  # For TailwindCSS processing
   ```

4. **Set up environment variables**:
   ```bash
   cp .env.example .env
   ```
   
   Configure your `.env` file:
   ```env
   # Azure Cosmos DB Configuration
   COSMOS_DB_URI=https://your-account.documents.azure.com:443/
   COSMOS_DB_KEY=your-primary-key
   COSMOS_DB_DATABASE=finance-tracker
   
   # Authentication
   JWT_SECRET=your-super-secret-jwt-key-at-least-32-characters
   
   # Server Configuration  
   SERVER_HOST=0.0.0.0
   SERVER_PORT=3000
   ```

5. **Run the development server**:
   ```bash
   cargo leptos watch
   ```

   Visit [http://localhost:3000](http://localhost:3000) to see your application!

## 🛠️ Development

### Building for Production

```bash
cargo leptos build --release
```

This generates:
- Server binary: `target/server/release/finance-tracker`
- Static assets: `target/site/`

### Running Tests

```bash
# Unit tests
cargo test

# End-to-end tests  
cargo leptos end-to-end

# Run E2E tests in release mode
cargo leptos end-to-end --release
```

### Development Tools

- **Hot reloading**: Changes are automatically reflected
- **TailwindCSS**: Styles are compiled on-the-fly
- **Type checking**: Full-stack type safety with Rust

## 🌐 Deployment

### Azure Static Web Apps (Recommended)

This application is optimized for deployment on Azure Static Web Apps with Azure Functions backend.

1. **Build the application**:
   ```bash
   cargo leptos build --release
   ```

2. **Deploy to Azure**:
   - Follow the Azure Static Web Apps deployment guide
   - Configure environment variables in the Azure portal
   - The build artifacts are in `target/site/` and `target/server/release/`

### Manual Deployment

For manual deployment to any server:

1. Copy the built files to your server:
   ```
   your-server/
   ├── finance-tracker          # Server binary
   └── site/                     # Static assets
       ├── pkg/
       ├── favicon.ico
       └── output.css
   ```

2. Set environment variables and run the binary.

## 🧪 Testing

### End-to-End Tests

E2E tests are located in `end2end/tests/` and use Playwright:

```bash
# Install E2E dependencies
cd end2end && npm install

# Run E2E tests
cargo leptos end-to-end
```

### Test Coverage

- ✅ Authentication flows
- ✅ Transaction management  
- ✅ Dashboard functionality
- ✅ API endpoints
- ✅ Database operations

## 📖 Blog Series

This project is part of a comprehensive blog series on Medium:

1. **Part 1**: Introduction and Project Setup
2. **Part 2**: Domain Modeling & Data Layer
3. **Part 3**: Handling Environment Variables, Application State & Token-Based Authentication

[📚 Read the full series on Medium →](https://medium.com/@mleppan23)

## 🛡️ Security

- **JWT Authentication** with secure token management
- **Password hashing** using bcrypt
- **Input validation** on all endpoints
- **CORS** configuration for production
- **Environment-based** configuration management

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📝 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Leptos](https://leptos.dev/) - The reactive web framework for Rust
- [Axum](https://github.com/tokio-rs/axum) - Ergonomic web framework built with Tokio and Tower
- [TailwindCSS](https://tailwindcss.com/) - Utility-first CSS framework
- [Azure Cosmos DB](https://azure.microsoft.com/en-us/products/cosmos-db) - Globally distributed database service

## 📞 Support

- 🐛 **Found a bug?** [Open an issue](https://github.com/your-username/finance-tracker/issues)
- 💬 **Have questions?** [Start a discussion](https://github.com/your-username/finance-tracker/discussions)
- 📖 **Documentation**: Check out the [blog series](https://medium.com/@your-username) for detailed guides

---

<div align="center">
  Made with ❤️ and 🦀 Rust
</div>