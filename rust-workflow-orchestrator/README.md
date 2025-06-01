# Rust Workflow Orchestrator

## Overview
Rust Workflow Orchestrator is a lightweight workflow orchestration system inspired by Apache Airflow. It allows users to define Directed Acyclic Graphs (DAGs) for scheduling and executing tasks. The system is built using Rust and leverages asynchronous programming for efficient task management.

## Project Structure
The project is organized into several modules:

- **scheduler**: Manages the scheduling of tasks based on defined DAGs and cron expressions.
- **executor**: Executes tasks either locally or in a distributed manner.
- **dag**: Defines the structure and parsing of DAGs.
- **database**: Manages metadata storage using SQLite.
- **web**: Provides a REST API for interacting with the orchestrator.
- **utils**: Contains utility functions, including logging.

## Getting Started

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation
1. Clone the repository:
   ```
   git clone https://github.com/microsoft/vscode-remote-try-rust.git
   cd rust-workflow-orchestrator
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the project:
   ```
   cargo run
   ```

### Configuration
DAGs can be defined in YAML format. An example DAG is provided in the `examples/sample_dag.yaml` file.

### Sample DAG
Here is a sample DAG definition with two tasks:

```yaml
name: Sample DAG
schedule: "0 * * * *"  # Every hour
tasks:
  - id: task1
    command: "echo 'Task 1'"
    dependencies: []
  - id: task2
    command: "echo 'Task 2'"
    dependencies: [task1]
```

### API Endpoints
- **GET /api/dags**: List all DAGs.
- **GET /api/task_instances**: View the status of task instances.
- **POST /api/dags/{dag_id}/run**: Trigger a manual run of a DAG.

## Error Handling and Logging
The project includes robust error handling and logging using the `log` crate and `env_logger`. Logs are output to the console for easy debugging.

## Contributing
Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.

## Acknowledgments
Thanks to the Rust community for their support and contributions to the ecosystem.