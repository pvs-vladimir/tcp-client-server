# TCP клиент-сервер

## Сервер

`cargo run -p server`

Представляет собой хранилище ограниченного числа задач, описываемых названием и ID.

Предоставляет клиентам следующий функционал посредством обмена json-сообщениями:

1. Добавить задачу:

    **Запрос:**

    ```json
    {"type": "add", "task_name": "task name"}
    ```

    **Ответ:**

    ```json
    {"response_status": "success", "task_id": 1}
    {"response_status": "fail", "message": "error message"}
    ```

2. Запросить задачу:

    **Запрос:**

    ```json
    {"type": "get", "task_id": 1}
    ```

    **Ответ:**

    ```json
    {"response_status": "success", "task_id": 1, "task_name": "task name"}
    {"response_status": "fail", "message": "error message"}
    ```

3. Удалить задачу:

    **Запрос:**

    ```json
    {"type": "delete", "task_id": 1}
    ```

    **Ответ:**

    ```json
    {"response_status": "success", "task_id": 1, "task_name": "task name"}
    {"response_status": "fail", "message": "error message"}
    ```

##  Клиент

Предоставляет пользователю консольный интерфейс:

- `cargo run -p client -- --command add --name "task name"`
- `cargo run -p client -- --command get --id 1`
- `cargo run -p client -- --command delete --id 1`
