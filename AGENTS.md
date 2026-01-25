# Minerva System - AGENTS.md

## Overview
This file serves as documentation for the Minerva System's agent-based architecture and service composition.

## Architecture Components

The Minerva System is composed of several Rust-based microservices that work together to form a multi-tenant ERP system:

### Core Services
- **minerva-rest**: REST gateway for client communication
- **minerva-user**: User management service
- **minerva-session**: Session handling service
- **minerva-data**: Data management service
- **minerva-cache**: Redis caching service
- **minerva-broker**: Message brokering service (RabbitMQ)
- **minerva-rpc**: Remote Procedure Call service
- **minerva-dispatch**: Task dispatching service
- **minerva-runonce**: One-time setup service

### Infrastructure Services
- **PostgreSQL 15**: Main relational database
- **MongoDB 4**: Document database
- **Redis 7**: Caching layer
- **RabbitMQ**: Message queue

## Agent Responsibilities

### Service Agents
Each microservice acts as an agent with specific responsibilities:
1. **minerva-rest**: Entry point for external requests and API gateway
2. **minerva-user**: User authentication, authorization, and profile management
3. **minerva-session**: Session management and security token handling
4. **minerva-data**: Data processing and storage operations
5. **minerva-cache**: Cache management and Redis connectivity
6. **minerva-broker**: Message queuing and inter-service communication
7. **minerva-rpc**: Remote procedure call handling
8. **minerva-dispatch**: Task scheduling and execution
9. **minerva-runonce**: Initial setup and one-time configuration

## Communication Patterns

### Inter-Service Communication
- Services communicate through RabbitMQ message broker
- REST API for client interactions
- Direct database connections for data operations
- Redis for caching and session storage

### Multi-Tenant Architecture
The system supports multi-tenant isolation through:
- Database namespacing
- User authentication and authorization layers
- Tenant-specific session management

## Deployment

### Local Development
See README.md for detailed instructions on local deployment.

### Production Deployment
- Docker-based deployment
- Kubernetes orchestration (refer to .github directory)
- Containerized microservices with proper resource allocation

## Configuration

### Environment Variables
Each service requires specific environment variables for proper operation:
- Database connection strings
- Cache server addresses
- Messaging queue credentials
- API gateway configurations

### Logging
Each service has its own logging configuration file located in deploy/logconfig directory.

## Technology Stack

### Backend
- Rust programming language
- Docker containerization
- Kubernetes orchestration
- Redis for caching
- PostgreSQL and MongoDB for data storage
- RabbitMQ for message queuing

### Frontend
- Flutter framework for mobile/desktop interface

## Update Protocol

When changes are made to any build, deployment, or development methods:
1. Always update the README.md file with the new procedures
2. Ensure all deployment instructions are accurate and working
3. Test the documented procedures before committing changes
4. Document any new prerequisites or requirements for building and deployment