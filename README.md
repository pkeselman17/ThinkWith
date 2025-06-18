# ThinkWith

A web application for collaborative brainstorming and idea management.

## Features

- Real-time collaboration
- Idea boards and voting
- Commenting and feedback

## Getting Started

### Prerequisites

- Node.js >= 18.x
- npm or yarn
- Rust >= 1.8
- Docker 

### Installation

App
```bash
git clone https://github.com/yourusername/thinkwith.git
cd thinkwith
npm install
```
Server
```bash
cd thinkwith_server
docker-compose up -d
cargo build
```

### Running the App

```bash
npm run start
npm run dev #to run in dev with turbopack
```

Visit `http://localhost:3000` in your browser.

### Running the Server

```bash
cargo run
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/YourFeature`)
3. Commit your changes
4. Push to the branch
5. Open a pull request

## License

MIT

## Contact

For questions or feedback, open an issue