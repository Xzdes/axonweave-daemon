### README.md

(Скопируй все содержимое ниже в файл `README.md` в корневой директории твоего проекта)

# Axonweave

**[English](#english) | [Русский](#русский)**

---

## English

### Axonweave: High-Performance Local Microservice Bridge

**Axonweave** is a lightweight, ultra-fast, and secure bridge for inter-process communication (IPC) between microservices running on a single machine. Written in Rust, it provides a central hub for service discovery and message proxying, allowing applications written in different programming languages (Python, Go, Node.js, etc.) to communicate seamlessly and efficiently as if they were part of a single monolithic application.

The core idea is to move away from slow, text-based protocols like HTTP/JSON for local communication and leverage the full power of gRPC over high-speed transports, managed by a reliable Rust core.

### Core Features

*   **High Performance:** Built on Rust and Tokio, ensuring minimal latency and low resource consumption. Designed to use Unix Sockets / Named Pipes in the future for maximum speed.
*   **Service Discovery:** Services dynamically register with the Axonweave daemon, making them instantly available to other registered services. No static configuration needed.
*   **Centralized Proxy:** Acts as a smart proxy, routing gRPC calls from one service to another without them needing to know each other's addresses.
*   **Resilience:** Includes a "reaper" mechanism that automatically de-registers services that have become unresponsive, keeping the service mesh clean.
*   **Language Agnostic:** Any language with gRPC support can integrate with Axonweave.
*   **Self-Contained:** The Axonweave daemon is a single binary with no external runtime dependencies.

### How It Works

The Axonweave ecosystem consists of three main parts:

1.  **The Daemon (`axonweave-daemon`):** The central Rust application that runs as a background process. It listens for incoming connections and manages the service registry.
2.  **The Services (e.g., `echo_server`):** Your actual microservices (written in any language). They run as separate processes and register themselves with the daemon upon startup.
3.  **The Clients (e.g., `proxy_test_client`):** Any service that wants to call another service. It does so by sending a special proxy request to the Axonweave daemon, which then forwards the call to the correct target.

![Architecture Diagram Placeholder](https://via.placeholder.com/800x400.png?text=Client+->+Axonweave+->+Target+Service)

### How to Use This Project (Developer's Guide)

This repository contains the source code for the `axonweave-daemon` and several usage examples.

#### Prerequisites

*   Rust toolchain (`rustup`, `cargo`) installed.
*   (Optional, for development) `protoc` compiler if you want to modify the `.proto` API files.

#### 1. Build the Project

Clone the repository and build all components:

```bash
git clone https://your-repo-url/axonweave-daemon.git
cd axonweave-daemon
cargo build --release
```
This will create optimized binaries in the `target/release/` directory.

#### 2. Run the Full Demo

The best way to see Axonweave in action is to run the full demo, which requires **three separate terminal windows**.

**Terminal 1: Start the Axonweave Daemon**

This is the core of the system.

```bash
cargo run --release --bin axonweave-daemon
```
You will see logs indicating that the daemon is listening on `127.0.0.1:50051`.

**Terminal 2: Start the Target Microservice (`echo_server`)**

This is a simple gRPC service that will be our destination.

```bash
cargo run --release --bin echo_server
```
It will start listening on a different port, `127.0.0.1:50052`.

**Terminal 3: Run the Proxy Client**

This client will perform the full sequence: register the echo server with the daemon, and then call it via the daemon's proxy.

```bash
cargo run --release --bin proxy_test_client
```

#### Expected Outcome

*   **Daemon Log (Terminal 1):** You will see logs for a new service registration, followed by logs for a proxy call.
    ```
    [Registry] Registering: 'echo.v1.EchoService'...
    [Proxy] Got a UnaryProxy call for path: /echo.v1.EchoService/SayHello
    [Proxy] Connecting to target service 'echo.v1.EchoService' at http://127.0.0.1:50052
    ```
*   **Echo Server Log (Terminal 2):** You will see that it received a request.
    ```
    [EchoServer] Received a 'SayHello' request with name: 'ProxiedUser'
    ```
*   **Client Log (Terminal 3):** You will see a successful response that was proxied through the daemon.
    ```
    [ProxyClient] SUCCESS! Received proxied response:
    EchoResponse {
        message: "Hello, ProxiedUser! This is EchoServer.",
    }
    ```

### Next Steps & Future Development

The core functionality is complete! The next steps will focus on creating easy-to-use SDKs (client libraries) for other languages like Python and Go. These SDKs will hide the complexity of the proxy mechanism, allowing developers to make calls like `echo_client.say_hello()` and have the library automatically route them through Axonweave.

---

<a name="русский"></a>
## Русский

### Axonweave: Высокопроизводительный локальный мост микросервисов

**Axonweave** — это легковесный, сверхбыстрый и безопасный мост для межпроцессного взаимодействия (IPC) между микросервисами, запущенными на одной машине. Написанный на Rust, он представляет собой центральный узел для обнаружения сервисов и проксирования вызовов, позволяя приложениям на разных языках (Python, Go, Node.js и т.д.) общаться друг с другом эффективно и бесшовно, как если бы они были частью одного монолитного приложения.

Ключевая идея — уйти от медленных текстовых протоколов, таких как HTTP/JSON, для локального взаимодействия и использовать всю мощь gRPC поверх высокоскоростных каналов связи, управляемых надежным ядром на Rust.

### Ключевые особенности

*   **Высокая производительность:** Построен на Rust и Tokio, что обеспечивает минимальные задержки и низкое потребление ресурсов. В будущем спроектирован для использования Unix Sockets / Named Pipes для максимальной скорости.
*   **Обнаружение сервисов:** Сервисы динамически регистрируются в демоне Axonweave, что делает их мгновенно доступными для других зарегистрированных сервисов. Не требует статической конфигурации.
*   **Централизованный прокси:** Выступает в роли умного прокси, маршрутизируя gRPC-вызовы от одного сервиса к другому без необходимости им знать адреса друг друга.
*   **Отказоустойчивость:** Включает механизм "жнеца" (reaper), который автоматически удаляет из реестра сервисы, переставшие отвечать, поддерживая чистоту сервисной сетки.
*   **Языковая независимость:** Любой язык с поддержкой gRPC может интегрироваться с Axonweave.
*   **Автономность:** Демон Axonweave представляет собой единый бинарный файл без внешних зависимостей времени выполнения.

### Как это работает

Экосистема Axonweave состоит из трех основных частей:

1.  **Демон (`axonweave-daemon`):** Центральное приложение на Rust, работающее как фоновый процесс. Он прослушивает входящие соединения и управляет реестром сервисов.
2.  **Сервисы (например, `echo_server`):** Ваши реальные микросервисы (написанные на любом языке). Они запускаются как отдельные процессы и регистрируют себя в демоне при старте.
3.  **Клиенты (например, `proxy_test_client`):** Любой сервис, который хочет вызвать другой сервис. Он делает это, отправляя специальный прокси-запрос демону Axonweave, который затем перенаправляет вызов на правильную цель.

![Схема архитектуры](https://via.placeholder.com/800x400.png?text=Клиент+->+Axonweave+->+Целевой+сервис)

### Как использовать проект (Руководство для разработчика)

Этот репозиторий содержит исходный код демона `axonweave-daemon` и несколько примеров использования.

#### Требования

*   Установленный инструментарий Rust (`rustup`, `cargo`).
*   (Опционально, для разработки) Компилятор `protoc`, если вы хотите изменять файлы API `.proto`.

#### 1. Сборка проекта

Склонируйте репозиторий и соберите все компоненты:

```bash
git clone https://your-repo-url/axonweave-daemon.git
cd axonweave-daemon
cargo build --release
```
Эта команда создаст оптимизированные бинарные файлы в директории `target/release/`.

#### 2. Запуск полного демо

Лучший способ увидеть Axonweave в действии — запустить полное демо, для которого потребуется **три отдельных окна терминала**.

**Терминал 1: Запуск демона Axonweave**

Это ядро системы.

```bash
cargo run --release --bin axonweave-daemon
```
Вы увидите логи, сообщающие, что демон слушает адрес `127.0.0.1:50051`.

**Терминал 2: Запуск целевого микросервиса (`echo_server`)**

Это простой gRPC-сервис, который будет нашей целью.

```bash
cargo run --release --bin echo_server
```
Он начнет слушать другой порт, `127.0.0.1:50052`.

**Терминал 3: Запуск прокси-клиента**

Этот клиент выполнит полную последовательность: зарегистрирует эхо-сервер в демоне, а затем вызовет его через прокси-механизм демона.

```bash
cargo run --release --bin proxy_test_client
```

#### Ожидаемый результат

*   **Лог демона (Терминал 1):** Вы увидите запись о регистрации нового сервиса, а затем запись о прокси-вызове.
    ```
    [Registry] Registering: 'echo.v1.EchoService'...
    [Proxy] Got a UnaryProxy call for path: /echo.v1.EchoService/SayHello
    [Proxy] Connecting to target service 'echo.v1.EchoService' at http://127.0.0.1:50052
    ```
*   **Лог эхо-сервиса (Терминал 2):** Вы увидите, что он получил запрос.
    ```
    [EchoServer] Received a 'SayHello' request with name: 'ProxiedUser'
    ```
*   **Лог клиента (Терминал 3):** Вы увидите успешный ответ, который был проксирован через демон.
    ```
    [ProxyClient] SUCCESS! Received proxied response:
    EchoResponse {
        message: "Hello, ProxiedUser! This is EchoServer.",
    }
    ```

### Следующие шаги и будущее развитие

Основной функционал завершен! Следующие шаги будут сосредоточены на создании простых в использовании SDK (клиентских библиотек) для других языков, таких как Python и Go. Эти SDK скроют сложность прокси-механизма, позволяя разработчикам делать вызовы вида `echo_client.say_hello()`, а библиотека будет автоматически маршрутизировать их через Axonweave.