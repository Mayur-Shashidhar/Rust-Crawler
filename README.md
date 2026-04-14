# 🕷️ Rust Crawler — Concurrent Web Crawler in Rust

A high-performance **asynchronous web crawler** built in Rust that recursively crawls web pages, extracts metadata, collects links, and stores structured results in JSON.
Designed to demonstrate **Tokio async concurrency, HTML parsing, BFS crawling, and safe parallel execution**.

---

# 🚀 Features

* ⚡ Concurrent crawling using Tokio async runtime
* 🧵 Worker pool with concurrency limit
* 🔁 Recursive BFS crawling
* 🚫 Duplicate URL detection
* 📊 Live progress bar
* 🔗 Extract links from each page
* 📝 Extract page title
* 📄 Extract meta description
* 📦 JSON dataset export
* 🛡️ Error-safe scraping (no crashes)
* 🌐 Relative URL resolution
* ⏱️ Timeout + request headers
* 🎯 Configurable max pages

---

# 🧠 How It Works

RustCrawler performs **breadth-first crawling**:

```
Seed URLs
   ↓
Fetch page
   ↓
Extract links
   ↓
Add to queue
   ↓
Repeat until max pages reached
```

The crawler:

* maintains a **visited set**
* uses **queue-based BFS**
* runs **N concurrent workers**
* collects structured page data

---

# 📁 Project Structure

```
rust-crawler/
│
├── src/
│   └── main.rs
│
├── results.json
├── Cargo.toml
└── README.md
```

---

## 🧰 Tech Stack

* **Rust** — core language
* **Tokio** — async runtime & concurrency
* **Reqwest** — HTTP client
* **Scraper** — HTML parsing
* **Serde / serde_json** — JSON serialization
* **Futures** — async task handling
* **Indicatif** — progress bar
* **url** — URL normalization & resolution

---

# 📦 Dependencies/Crates Used

```
tokio
reqwest
scraper
serde
serde_json
futures
indicatif
url
```

These provide:

* async runtime
* HTTP client
* HTML parsing
* JSON serialization
* concurrency utilities
* progress bar
* URL normalization

---

# ⚙️ Installation

### 1. Install Rust

```
https://rustup.rs
```

Verify:

```bash
rustc --version
cargo --version
```

---

### 2. Clone Project

```bash
git clone <repo-url>
cd rust-crawler
```

---

### 3. Install Dependencies

```bash
cargo build
```

---

# ▶️ Running the Crawler

```bash
cargo run
```

Example output:

```
[00:01:22] ████████████████████████████████████████ 50/50
Saved 50 pages to results.json
```

---

# 📄 Output Format

The crawler generates:

```
results.json
```

Example:

```json
[
  {
    "url": "https://example.com",
    "title": "Example Domain",
    "description": "Example description...",
    "links": [
      "https://example.com/about",
      "https://example.com/contact"
    ]
  }
]
```

---

# 🔍 Data Collected

Each page includes:

| Field       | Description          |
| ----------- | -------------------- |
| url         | crawled page URL     |
| title       | HTML `<title>`       |
| description | meta description     |
| links       | extracted hyperlinks |

---

# 🧵 Concurrency Model

RustCrawler uses:

* Tokio async runtime
* Semaphore worker limit
* join_all batch execution
* Shared queue (Mutex)
* Shared visited set

Execution:

```
Worker 1 ┐
Worker 2 ├── fetch concurrently
Worker 3 │
Worker 4 │
Worker 5 ┘
```

---

# 🌐 Crawling Strategy

* BFS traversal
* queue-based scheduling
* deduplicate visited URLs
* resolve relative links
* limit max pages

---

# 🎯 Configuration

Edit in `main.rs`:

```rust
let max_pages: usize = 50;
let concurrency = 5;
```

Example:

```rust
let max_pages = 200;
let concurrency = 10;
```

---

# ⚡ Performance

| Pages     | Time     |
| --------- | -------- |
| 50 pages  | ~1–2 min |
| 100 pages | ~3 min   |
| 500 pages | ~10 min  |

Depends on:

* network
* concurrency
* site latency

---

# 🛡️ Error Handling

Crawler safely handles:

* request failures
* invalid URLs
* timeout errors
* blocked sites
* missing titles
* broken links

No panics.

---

# 🧪 Example Seeds

Edit seeds in `main.rs`:

```rust
let seeds = vec![
    "https://example.com",
    "https://www.rust-lang.org",
];
```

---

# 📊 Use Cases

* Search engine crawler
* Dataset builder
* SEO analysis
* Link graph generation
* Metadata extraction
* Web scraping automation
* ML dataset generation

---

# 🏗️ Architecture

```
Seed URLs
   ↓
Queue
   ↓
Workers (async)
   ↓
Fetch HTML
   ↓
Parse DOM
   ↓
Extract metadata
   ↓
Extract links
   ↓
Push new URLs
   ↓
Repeat
```

---

# 🚀 Future Improvements

* Domain-only crawling
* Depth limit
* Robots.txt support
* Keyword search index
* SQLite storage
* REST API
* CLI arguments
* Distributed crawler
* Page content extraction
* Image scraping

---

# 🧵 Technologies Used

* Rust
* Tokio
* Reqwest
* Scraper
* Serde
* Indicatif
* Futures
* URL crate

---

# 📈 Learning Outcomes

This project demonstrates:

* async Rust
* concurrency control
* BFS graph traversal
* web scraping
* shared state synchronization
* error handling
* structured data export
* multi-threaded architecture
