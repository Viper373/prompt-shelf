# Prompt Shelf

A prompt management system with version control capabilities, built with Rust and modern web technologies.

## Features

- Create and manage prompts with version history tracking
- Support for multiple versions (nodes) and commits
- RESTful API interface for integration
- Dockerized deployment with MySQL and Dragonfly (Redis-compatible) database
- JWT authentication for secure access

## Quick Start

### Prerequisites
- Docker and Docker Compose installed

### Installation

1. Clone this repository
2. Navigate to the project directory:
   ```bash
   cd prompt-shelf
   ```
3. Start the services using Docker Compose:
   ```bash
   docker-compose up --build -d
   ```
4. The API server will be available at http://localhost:8000

## Environment Configuration

The following environment variables can be configured in the docker-compose.yml file:

- `MYSQL_URI`: MySQL connection string
- `REDIS_URI`: Dragonfly/Redis connection string
- `JWT_SECRET`: JWT signing secret
- `JWT_EXPIRATION`: JWT expiration time (seconds)
- `ALLOW_REGISTER`: Allow user registration (true/false)

## API Documentation

For detailed API documentation, please refer to the [HTML documentation](./doc/PromptShelf.html)

### Key API Endpoints Summary

### Authentication

| Method | Endpoint           | Description                  |
|--------|--------------------|------------------------------|
| POST   | /user/signin       | User login                   |
| POST   | /user/signup       | User registration            |

### Prompt Management

| Method | Endpoint                 | Description                  |
|--------|--------------------------|------------------------------|
| POST   | /prompt/create_prompt    | Create a new prompt          |
| POST   | /prompt/create_node      | Create a new version node    |
| POST   | /prompt/create_commit    | Commit changes to a prompt   |
| GET    | /prompt/query            | Query prompts                |
| GET    | /prompt/latest           | Get latest prompt version    |
| GET    | /prompt/content          | Get prompt content           |
| POST   | /prompt/rollback         | Rollback to previous version |
| POST   | /prompt/revert           | Revert changes               |
| DELETE | /prompt/                 | Delete a prompt              |

### System

| Method | Endpoint           | Description                  |
|--------|--------------------|------------------------------|
| GET    | /status            | Check service health status  |

### Admin Control

| Method | Endpoint                | Description                          |
|--------|-------------------------|--------------------------------------|
| POST   | /control/register       | Enable/disable user registration     |
| GET    | /control/list/user      | List all users (admin only)          |

## Project Structure

```
prompt-shelf/
├── src/                 # Rust backend source
│   ├── db/              # Database models
│   ├── routes/          # API route handlers
│   └── main.rs          # Application entry point
├── app/                 # Frontend application
├── conf/                # Configuration files
├── docker-compose.yml   # Docker Compose configuration
└── Cargo.toml           # Rust dependencies
```

## Technology Stack

- **Backend**: Rust, Axum, SeaORM
- **Database**: MySQL, Dragonfly (Redis)
- **Authentication**: JWT
- **Containerization**: Docker, Docker Compose

## License

[MIT](LICENSE)