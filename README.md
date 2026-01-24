# Minerva System

Source code for the Minerva System, a bare minimum study on ERP solutions
using microservices and a multi-tenant architecture.

## Description

The Minerva System is a prototype of an ERP with bare minimum systems running,
but with room for improvement. It is not built as a serious competitor to any
other system **whatsoever**, instead being thought of as a case study for the
possible technological applications of *bleeding-edge* technology for such a
use case.

As an overview, the Minerva System is an attempt at building a microservice
architecture for a bare minimum ERP, using the best that Rust, Docker,
Kubernetes, Redis and Flutter have to offer.

From the *back-end* standpoint, it has a REST gateway from which a client may
interface with the service itself, which is an array of microservices
communicating and interfacing with PostgreSQL and MongoDB databases.

From the *front-end* standpoint, it is mostly a study on using Flutter and
*bleeding-edge* UX design patterns.

Furthermore, for isolation purposes, it is also thought of as a *multi-tenant*
application that can work with multiple databases. This way, both PostgreSQL and
MongoDB shall provide a namespaced solution on a client basis.

Since this is quite a complex application, do not expect it to be easy to deploy,
and do not expect it to be fully documented (though I'm trying my best to do so).
Also, **do not use it for commercial purposes**, unless explicitly given permission
(**which will never happen, so as blunt it may seem to hear, give up**).

Think of this system as a big reference for some strategies on the given technologies,
though there are no guarantees that those strategies are necessarily correct.
Therefore, mostly use this as a reference implementation depending on the strategy
you want to adopt for any given technology.

## Documentation

If you wish to read the online documentation, please refer to
the [Minerva System documentation](https://luksamuk.github.io/minerva-system),
which is currently written in Brazillian Portuguese.

## Running Locally

To run the Minerva System locally, you need to start the required services and then the microservices.

### Prerequisites
- Docker installed
- Docker Compose installed

### Step-by-step Instructions

1. **Start the infrastructure services**:
   ```bash
   cd deploy/compose/services
   docker-compose up -d
   ```
   This will start:
   - PostgreSQL 15 database (port 5432)
   - MongoDB 4 database (port 27017)
   - Redis 7 cache (port 6379)
   - RabbitMQ with management UI (ports 5672, 15672)

2. **Start the microservices**:
   ```bash
   cd deploy/compose
   docker-compose up -d
   ```

3. **Access the system**:
   - REST API: http://localhost:9000/api
   - RabbitMQ Management UI: http://localhost:15672 (username: rabbitmq, password: minerva)

### Service Credentials
- PostgreSQL: user=postgres, password=postgres
- MongoDB: user=root, password=mongo
- Redis: no password
- RabbitMQ: user=rabbitmq, password=minerva

## Licensing

This project is licensed under the GPLv3 license. For more information, see
the LICENSE file, or refer to [Choose a License.com](https:///choosealicense.com).

