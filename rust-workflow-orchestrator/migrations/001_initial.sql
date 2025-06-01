CREATE TABLE dags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    schedule TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dag_id INTEGER NOT NULL,
    task_id TEXT NOT NULL,
    command TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (dag_id) REFERENCES dags(id)
);

CREATE TABLE task_instances (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER NOT NULL,
    execution_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    state TEXT NOT NULL CHECK(state IN ('pending', 'running', 'completed')),
    FOREIGN KEY (task_id) REFERENCES tasks(id)
);