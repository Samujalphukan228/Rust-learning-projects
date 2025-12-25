# 🦀 Todo API but it's Rust

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-EC5800?style=for-the-badge&logo=rust&logoColor=white)
![MongoDB](https://img.shields.io/badge/MongoDB-47A248?style=for-the-badge&logo=mongodb&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-000000?style=for-the-badge&logo=rust&logoColor=white)

**Because JavaScript is too mainstream and we like our memory safe** 🔒

[Features](#-features) • [Quick Start](#-quick-start) • [API Docs](#-api-endpoints) • [Deployment](#-deployment)

</div>

---

## 🤔 What is this?

A **dead simple Todo API** built with Rust.  
Fast, memory-safe, and good for flexing at meetups.

```rust
fn main() {
    println!("Fast, Safe, Sexy 🚀");
}
```

---

## 🍔 Tech Stack

- 🦀 **Rust**
- 🎯 **Axum**
- 🍃 **MongoDB**
- ⚡ **Tokio**

---

## 🚀 Quick Start

### 1️⃣ Clone the repo

```bash
git clone https://github.com/yourusername/rust-todo-api
cd rust-todo-api
```

### 2️⃣ Create `.env`

```env
MONGODB_URI=mongodb://localhost:27017
DB_NAME=todo_app
PORT=3000
```

### 3️⃣ Run it

```bash
cargo run
```

Server runs at:  
👉 http://localhost:3000

---

## 📋 API Endpoints

| Method | Endpoint        | Description        |
|------:|-----------------|--------------------|
| GET   | /todos          | Get all todos      |
| POST  | /todos          | Create a todo      |
| GET   | /todos/:id      | Get one todo       |
| PUT   | /todos/:id      | Update a todo      |
| DELETE| /todos/:id      | Delete a todo      |

---

## 📬 Example Requests

### Create a todo

```bash
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"Touch grass"}'
```

Response:

```json
{
  "_id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Touch grass",
  "done": false
}
```

---

### Get all todos

```bash
curl http://localhost:3000/todos
```

---

### Mark as done

```bash
curl -X PUT http://localhost:3000/todos/<id> \
  -H "Content-Type: application/json" \
  -d '{"done": true}'
```

---

### Delete a todo

```bash
curl -X DELETE http://localhost:3000/todos/<id>
```

---

## 📁 Project Structure

```
src/
├── main.rs
├── handlers/
│   ├── create.rs
│   ├── read.rs
│   ├── update.rs
│   └── delete.rs
├── models/
│   └── todo.rs
├── routes/
│   └── mod.rs
└── db.rs
```

---

## 🍃 MongoDB Setup

### Local

**macOS**
```bash
brew install mongodb-community
brew services start mongodb-community
```

**Linux**
```bash
sudo apt install mongodb
sudo systemctl start mongod
```

**Windows**
Download the installer from MongoDB.

### Cloud (Atlas)

1. Create a free cluster
2. Get the connection string
3. Update `.env`

---

## 📦 Cargo.toml

```toml
[package]
name = "todo-api"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["cors"] }
mongodb = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
dotenvy = "0.15"
futures = "0.3"
```

---

## 📝 Todo Model

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub done: bool,
}
```

JSON:

```json
{
  "_id": "some-uuid",
  "title": "Learn Rust",
  "done": false
}
```

---

## 🚢 Deployment

### Easy Mode
- Railway / Render
- Set env vars
- Deploy

### VPS Mode

```bash
cargo build --release
scp target/release/todo-api root@server:/app
ssh root@server
./todo-api
```

---

## 🤝 Contributing

PRs welcome:
- 🐛 Bugs
- ✨ Features
- 🎯 Improvements

---

## 📜 License

MIT

---

<div align="center">
Made with 🦀 and ☕  
Star the repo if it helped ⭐
</div>
