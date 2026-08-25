# Rust: Beginner to Expert — Comprehensive Project Roadmap

Yes. But I would **not** structure this as “learn Rust syntax → build a web server → build a compiler → congratulations, expert.”

That approach has a serious flaw: it teaches *Rust projects*, not *software engineering*. Your goal is broader: Rust itself, systems programming, algorithms, architecture, concurrency, networking, databases, distributed systems, security, testing, performance, tooling, language design, and the engineering practices that make code production-grade.

I’ve therefore designed the roadmap around **capability progression** rather than project difficulty alone.

One important current-version note: as of **August 25, 2026, Rust 1.98.0 is the latest stable release**, released August 20, 2026. Rust 2024 is the current edition, and Cargo’s modern workspace setup uses resolver 3.

# The roadmap

Think of this as **5 stages and 30 major projects**.

You should not merely finish each project. For the later projects, you should be able to **design, benchmark, test, document, secure, deploy, and defend the design decisions**.

| Stage | Projects | Main transformation |
|---|---:|---|
| I. Foundations | 1–6 | Python/JS programmer → Rust programmer |
| II. Systems & Concurrency | 7–12 | Rust programmer → systems programmer |
| III. Production Engineering | 13–19 | systems programmer → production engineer |
| IV. Advanced Systems | 20–25 | programmer → advanced Rust engineer |
| V. Expert / Language & Infrastructure | 26–30 | advanced engineer → systems/language specialist |

---

# Stage I — Foundations

These projects are deliberately smaller than the projects you're probably excited about.

That's intentional.

Your first objective is to stop translating Python/TypeScript into Rust.

You need to start **thinking in ownership, borrowing, algebraic data types, iterators, traits and explicit error handling**.

## 1. CLI Unit Converter

Build a serious command-line converter.

Examples:

```text
length
temperature
mass
speed
time
data
currency
```

Do not make it one giant `match`.

Learn:

- variables and mutability
- scalar types
- tuples
- structs
- enums
- pattern matching
- `Option`
- `Result`
- functions
- modules
- ownership
- borrowing
- slices
- `String` vs `&str`
- iterators
- basic generics

### Stretch

Create a reusable library crate underneath the CLI.

Your CLI becomes one consumer of your library.

That's your first introduction to **API design**.

---

# 2. Todo CLI

Build a local task manager.

Features:

```text
todo add "Learn lifetimes"
todo list
todo done 3
todo delete 3
todo edit 3
todo search rust
```

Store data in JSON or your own format.

Learn:

- serialization
- file I/O
- filesystem APIs
- error propagation
- custom errors
- `Display`
- `Debug`
- `From`
- iterators
- modules
- separation of concerns
- CLI argument parsing
- configuration
- testing

### Engineering requirement

No:

```rust
unwrap()
expect()
panic!()
```

in normal application paths unless there is a documented invariant explaining why the operation cannot fail.

Start developing the habit of asking:

> “What does the program do when this fails?”

That mindset is much more important than memorizing Rust syntax.

---

# 3. Text Search Engine

Build a small `grep`/Ripgrep-style utility.

Example:

```bash
rgrep "hello" ./src
```

Then add:

- recursive traversal
- regex
- case-insensitive search
- line numbers
- multiple files
- binary-file detection
- parallel searching

Learn:

- iterators
- lifetimes
- generics
- trait bounds
- ownership across APIs
- filesystem traversal
- concurrency
- channels
- performance measurement

### Stretch

Create:

```text
rgrep-core
rgrep-cli
```

You are now using a workspace-style architecture. Cargo workspaces are the standard mechanism for structuring multiple related crates.

---

# 4. In-Memory Database

Build your own tiny database.

Something like:

```sql
CREATE TABLE users ...
INSERT ...
SELECT ...
UPDATE ...
DELETE ...
```

Don't worry about making SQL fully compatible.

Implement:

- tables
- rows
- columns
- typed values
- indexes
- filtering
- sorting
- simple query parsing

Learn:

- enums as algebraic data types
- trait-oriented design
- generics
- iterators
- ownership
- data structures
- parsing
- error types
- API design

### Huge lesson

You will discover that a lot of “database programming” is actually **data modeling + invariants + algorithms + memory management**.

---

# 5. Expression Language

Make a tiny programming language.

Example:

```text
let x = 10;
let y = x * 4 + 2;
print(y);
```

Eventually:

```text
fn square(x) {
    x * x
}

print(square(10));
```

Build:

```text
lexer
   ↓
parser
   ↓
AST
   ↓
interpreter
```

Learn:

- recursive data structures
- `Box`
- `Rc` / `Arc`
- tree structures
- parsing
- recursive descent
- pattern matching
- environments
- scopes
- closures
- runtime errors

This is one of the most important projects in the entire curriculum.

---

# 6. Your Own Generic Collections

Implement versions of:

```text
Vec
VecDeque
HashMap
HashSet
Linked structure
Stack
Queue
Binary heap
```

Start entirely with safe Rust.

Then understand how the standard library implementations differ.

Learn:

- generics
- traits
- iterators
- ownership
- allocation
- `MaybeUninit`
- pointer concepts
- amortized complexity
- memory layout

Do **not** rush into `unsafe`.

The purpose is first to understand exactly what safe Rust buys you.

---

# Stage II — Systems & Concurrency

Now we start exploiting the things that attracted you to Rust in the first place.

---

# 7. Multithreaded Web Server

Build a TCP HTTP server manually.

Start:

```text
TCP
 ↓
HTTP parser
 ↓
router
 ↓
handler
```

Then add:

```text
thread pool
connection limits
timeouts
logging
graceful shutdown
```

Learn:

- TCP sockets
- HTTP
- protocol parsing
- routing
- threads
- synchronization
- resource limits
- graceful shutdown

The official Rust book itself uses a multithreaded web server as its capstone, covering TCP, HTTP parsing, responses and a thread pool.

But yours should eventually go significantly beyond the book version.

---

# 8. Thread Pool

Build the thread pool separately.

Support:

```rust
pool.execute(...)
pool.shutdown(...)
```

Then experiment with:

- bounded queues
- work stealing
- task cancellation
- worker starvation
- graceful shutdown
- panic recovery

Learn:

- `Send`
- `Sync`
- mutexes
- condition variables
- atomics
- channels
- memory ordering

`Send` and `Sync` are fundamental to Rust's concurrency model, and incorrectly implementing them in unsafe code can create undefined behavior.

---

# 9. Async Runtime

Now do something much harder:

**build a miniature Tokio-like runtime.**

Implement progressively:

```text
Future
 ↓
Waker
 ↓
Executor
 ↓
Spawner
 ↓
Timer
 ↓
Socket readiness
```

Eventually:

```rust
spawn(async {
    ...
});
```

This project will force you to understand:

- `Future`
- `Poll`
- `Pin`
- `Waker`
- task scheduling
- cooperative scheduling
- reactor/executor architecture
- async cancellation
- I/O readiness

This is one of the projects where the difference between “I know Rust” and “I understand Rust” becomes obvious.

---

# 10. Memory Allocator

Write an allocator.

Start with:

```text
bump allocator
```

Then:

```text
free-list allocator
```

Eventually:

```text
segregated free lists
```

Learn:

- heap allocation
- alignment
- raw pointers
- ownership invariants
- allocation strategies
- fragmentation
- `unsafe`
- memory layout

This is your first serious entry into unsafe Rust.

---

# 11. Shell

Build a Unix-like shell.

Support:

```bash
echo hello
cat file
ls
pwd
cd
```

Then:

```bash
command1 | command2
command > file
command < file
command &
```

Add:

- environment variables
- process spawning
- signals
- pipelines
- redirection
- job control

This teaches you something pure Rust tutorials often don't:

> Your program ultimately lives on top of an operating system.

You need to understand the boundary between:

```text
Rust
↓
libc / platform APIs
↓
kernel
↓
hardware
```

---

# 12. Concurrent Web Crawler

Build a crawler that can process thousands of URLs.

Requirements:

- bounded concurrency
- deduplication
- DNS
- HTTP
- retries
- backoff
- robots handling
- persistence
- cancellation
- graceful shutdown

Now you're combining:

```text
networking
+
async
+
concurrency
+
storage
+
parsing
+
fault tolerance
```

This is the transition from “projects” to **systems**.

---

# Stage III — Production Engineering

At this point you know considerably more Rust.

Now we stop optimizing for “learning Rust” and start optimizing for:

> **building software other people can depend on.**

---

# 13. Production REST API

Build an API from scratch.

Example:

```text
POST   /users
GET    /users/:id
PATCH  /users/:id
DELETE /users/:id
```

Include:

- authentication
- authorization
- validation
- structured errors
- PostgreSQL
- migrations
- transactions
- connection pooling
- configuration
- logging
- tracing
- metrics
- health checks
- graceful shutdown
- rate limiting

You can use a production framework here.

The point is no longer “reinvent everything.”

It's:

> Can you build something professionally?

---

# 14. Authentication Service

Build a standalone authentication system.

Implement:

```text
registration
login
password hashing
sessions
refresh tokens
logout
password reset
email verification
MFA
```

Study:

- cryptography fundamentals
- hashing
- secure randomness
- token design
- session management
- replay attacks
- timing attacks
- authorization

Do **not** invent cryptographic primitives.

Instead, learn to correctly compose established primitives and libraries.

---

# 15. Job Queue

Create something analogous to:

```text
Redis Queue
RabbitMQ
BullMQ
SQS
```

Features:

```text
enqueue
dequeue
retry
delay
priority
dead-letter queue
visibility timeout
```

Then distributed workers:

```text
Producer
   ↓
Queue
   ↓
Worker 1
Worker 2
Worker 3
```

Learn:

- distributed coordination
- retries
- idempotency
- delivery semantics
- backpressure
- failure handling

---

# 16. Key-Value Database

Build your own Redis-like database.

Commands:

```text
SET
GET
DEL
EXPIRE
TTL
INCR
```

Add:

- append-only log
- snapshotting
- persistence
- expiration
- concurrent clients
- transactions

Then benchmark it.

Now you're learning:

```text
network protocols
+
storage engines
+
serialization
+
concurrency
+
performance
```

---

# 17. HTTP Reverse Proxy

Build:

```text
client
   ↓
reverse proxy
   ↓
server cluster
```

Implement:

- load balancing
- health checks
- retries
- connection pooling
- timeouts
- TLS
- compression
- caching
- request logging
- rate limiting

Algorithms to experiment with:

```text
round robin
least connections
weighted round robin
consistent hashing
```

---

# 18. Distributed Cache

Build a Redis-like distributed cache.

Nodes:

```text
Node A
Node B
Node C
```

Implement:

- consistent hashing
- replication
- node membership
- failure detection
- rebalancing
- TTL
- partition handling

Now you encounter real distributed-systems problems.

---

# 19. Observability Platform

Build your own mini:

```text
OpenTelemetry
+
Prometheus
+
Jaeger
```

Collect:

```text
logs
metrics
traces
```

Your services should be instrumented and your observability platform should visualize them.

Learn:

- structured logging
- telemetry
- distributed tracing
- correlation IDs
- latency
- sampling
- cardinality
- monitoring architecture

This is often neglected by people learning programming.

Don't neglect it.

---

# Stage IV — Advanced Systems

Now things become substantially more difficult.

---

# 20. File System

Implement a userspace filesystem.

Build:

```text
virtual filesystem
directories
files
metadata
permissions
```

Then persist it to a block file.

Study:

- inode-style structures
- allocation maps
- journaling
- crash recovery
- caching
- locking

Eventually interface it with the OS where practical.

---

# 21. SQLite-Like Database

Now build a serious relational database.

Components:

```text
SQL lexer
SQL parser
planner
executor
storage engine
B-tree
buffer pool
transactions
locking
WAL
```

Eventually:

```sql
SELECT ...
FROM ...
JOIN ...
WHERE ...
GROUP BY ...
ORDER BY ...
```

This single project could occupy you for months.

And that's a feature, not a bug.

---

# 22. Compiler

Return to your earlier language, but now make it a real compiled language.

Pipeline:

```text
source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
HIR
 ↓
type checking
 ↓
MIR-like IR
 ↓
optimization
 ↓
machine code / VM
```

Add:

- generics
- pattern matching
- traits/interfaces
- closures
- modules
- borrow checking

Eventually attempt a **small borrow checker**.

This will dramatically deepen your understanding of Rust itself.

---

# 23. Static Analyzer / Linter

Build something resembling:

```text
Clippy
ESLint
Rust-analyzer
```

Analyze source code and detect:

```text
unused variables
dead code
complex expressions
possible bugs
style violations
security issues
```

Then create your own lint rules.

This teaches:

- ASTs
- semantic analysis
- source locations
- compiler architecture
- diagnostics
- developer tooling

---

# 24. Language Server

Build a mini LSP server.

Features:

```text
autocomplete
go-to-definition
hover information
diagnostics
rename
document symbols
```

Now you're dealing with the infrastructure behind modern IDEs.

---

# 25. WASM Runtime / VM

Implement a small WebAssembly interpreter.

Support progressively:

```text
modules
types
instructions
memory
tables
functions
imports/exports
```

Then embed your VM inside another Rust program.

This teaches:

- virtual machines
- binary formats
- interpreters
- execution engines
- sandboxing
- ABI concepts

---

# Stage V — Expert Territory

These projects are not “tutorial projects.”

They're **specialization projects**.

You do not need all of them to become an excellent Rust engineer, but doing several of them seriously will push you into expert territory.

---

# 26. Operating-System Kernel

Build a tiny kernel.

Start with:

```text
boot
memory management
interrupts
paging
allocator
processes
scheduler
filesystem
syscalls
```

Eventually:

```text
user space
shell
network stack
```

This teaches the deepest relationship between:

```text
language
compiler
CPU
memory
OS
process
```

---

# 27. Networking Stack

Implement pieces of a network stack yourself.

Start:

```text
Ethernet
IPv4
ARP
ICMP
UDP
TCP
```

Then:

```text
DNS
HTTP
TLS integration
```

You don't need to replace the OS network stack.

The objective is understanding the protocols.

---

# 28. Distributed Database

Take your database and distribute it.

Build:

```text
Leader
 ↓
Followers
```

Implement a consensus algorithm such as Raft.

Then:

```text
replication
leader election
log replication
snapshots
membership
failure recovery
```

Now you're doing serious distributed systems.

---

# 29. Production-Grade Rust Platform

This is the “everything project.”

Create a multi-crate workspace containing:

```text
CLI
API server
database
job queue
worker
authentication
observability
configuration
SDK
```

Deploy it.

Your repository should resemble a real company codebase.

For example:

```text
crates/
    core/
    domain/
    storage/
    api/
    auth/
    queue/
    telemetry/
    cli/
    sdk/

services/
    api/
    worker/
    scheduler/

tools/
    migration/
    benchmark/
```

Now enforce:

```text
CI
linting
formatting
tests
integration tests
benchmarks
fuzzing
security scanning
dependency auditing
documentation
semantic versioning
release automation
```

This is where programming becomes **software engineering**.

---

# 30. Build Something You Would Actually Ship

This final project should **not be chosen by me**.

Choose a difficult product you genuinely care about.

Examples:

```text
database
developer tool
payments engine
distributed storage system
privacy platform
game backend
message broker
desktop application
compiler
programming language
AI infrastructure
```

Then operate it like a real product.

That means:

```text
design
↓
implementation
↓
tests
↓
benchmarking
↓
security review
↓
deployment
↓
monitoring
↓
incident
↓
fix
↓
release
↓
maintenance
```

The final skill you need is not:

> “Can I build it?”

It's:

> “Can I keep it correct when reality attacks it?”

---

# But this still isn't enough

There are several areas that a simple project progression tends to miss.

So your roadmap should have **parallel tracks** running alongside the projects.

## Track A — Algorithms & Data Structures

You should implement and use:

```text
arrays
linked lists
stacks
queues
hash tables
trees
heaps
tries
graphs
disjoint sets
Bloom filters
LRU caches
B-trees
skip lists
```

Algorithms:

```text
sorting
binary search
graph traversal
shortest paths
topological sorting
dynamic programming
greedy algorithms
string algorithms
compression
hashing
```

Don't make LeetCode your primary education.

Implement the algorithms yourself first.

---

# Track B — Computer Science

Study:

```text
operating systems
computer architecture
networks
databases
distributed systems
compilers
cryptography
information theory
concurrency
parallelism
automata
type systems
```

Your Rust projects should force you to use these concepts.

---

# Track C — Mathematics

You don't need a mathematics degree.

But you should understand:

```text
logic
sets
probability
statistics
discrete mathematics
graphs
combinatorics
Big-O
amortized analysis
basic linear algebra
```

As your projects become more specialized, you'll naturally encounter more.

---

# Track D — Software Engineering

Every serious project must eventually have:

```text
Git
CI/CD
code review
semantic versioning
documentation
logging
testing
benchmarking
profiling
observability
security
dependency management
release management
```

Cargo workspaces, for example, let you split a growing system into independently understandable crates while maintaining shared workspace management.

---

# Track E — Testing

Don't stop at unit tests.

You need:

```text
unit tests
integration tests
property tests
fuzz tests
snapshot tests
stress tests
concurrency tests
end-to-end tests
benchmarks
```

Later:

```text
model checking
race detection
sanitizers
Miri
```

The mindset should become:

> Don't only test that the program works. Test the assumptions on which the program works.

---

# Track F — Performance

You should eventually become comfortable with:

```text
CPU cache
allocation
branch prediction
SIMD
parallelism
I/O
latency
throughput
contention
false sharing
memory locality
zero-copy design
```

For every serious project ask:

```text
Where is the bottleneck?
How do I know?
What measurement proves it?
Did the optimization actually help?
```

Never replace measurement with intuition.

---

# Track G — Unsafe Rust

Do not treat `unsafe` as “advanced Rust syntax.”

Treat it as a **formal engineering discipline**.

Eventually study:

```text
raw pointers
aliasing
validity
layout
alignment
FFI
ABI
uninitialized memory
variance
pinning
Send/Sync
atomics
memory ordering
concurrency invariants
```

The Rustonomicon explicitly frames unsafe Rust around constructing sound safe abstractions and understanding things like memory layout, FFI, concurrency, optimization and the relationship between Rust and lower-level machine/OS behavior.

Your rule should be:

> Prefer safe Rust. Isolate unsafe Rust. Document its invariants. Test it aggressively.

---

# Track H — APIs & Library Design

At some point you should stop thinking:

> “Does my code work?”

and start thinking:

> “Is my API pleasant to use?”

Rust's API guidelines explicitly emphasize naming conventions, conversions, iterator conventions, interoperability, common trait implementations and documentation/examples.

For every public library you create, ask:

```text
Is the API discoverable?
Are invalid states difficult to represent?
Are errors meaningful?
Are names idiomatic?
Are ownership expectations obvious?
Can this evolve without unnecessary breaking changes?
```

Rust's guidelines also emphasize validating inputs, preferably through types that make invalid states impossible where practical.

---

# The Rust standards I want you using throughout

Don't learn an old Rust style and then modernize it later.

Start modern.

As of now:

### Edition

Use:

```toml
edition = "2024"
```

The 2024 edition includes language and library changes such as `let` chains, changes to temporary scope, the `gen` keyword reservation, and additions to the prelude.

### Toolchain

Use `rustup`.

Keep stable current.

Rust 1.98.0 is currently the latest stable release.

### Formatting

Use Rustfmt.

### Linting

Use Clippy.

And don't treat it as merely a warning generator. Current Clippy contains hundreds of lints across correctness, performance, style, suspicious code, pedantic recommendations and more.

### Cargo

Become extremely comfortable with:

```text
cargo build
cargo check
cargo test
cargo bench
cargo doc
cargo fmt
cargo clippy
cargo tree
cargo metadata
cargo publish
cargo package
cargo update
```

And learn workspaces early.

### Documentation

Every serious library should have excellent rustdoc.

Rust's API guidelines explicitly encourage thorough crate-level documentation and examples for public APIs.

---

# One final adjustment: don't make this a “30-project checklist”

That's another trap.

I would divide your progression into **four levels of mastery**:

### Level 1 — Build

You can make it work.

### Level 2 — Understand

You can explain why it works.

### Level 3 — Engineer

You can make it reliable, tested, observable and maintainable.

### Level 4 — Defend

You can explain why your design is correct, why alternatives are worse, what its limits are, and how it behaves under pathological conditions.

For projects **1–6**, Level 2 is the goal.

For **7–19**, reach Level 3.

For **20–25**, start reaching Level 4.

For **26–30**, Level 4 should be the standard.

That distinction is much more important than the raw number of projects.

---

# The biggest mistake I would specifically warn you against

Because you already know Python and Next.js, you have a dangerous advantage.

You'll probably be able to write a Rust web API surprisingly quickly.

That can fool you into thinking:

> “I understand Rust.”

You don't.

You understand **how to make applications**.

Rust expertise requires understanding things you could often ignore in Python/TypeScript:

```text
ownership
borrowing
lifetimes
layout
allocation
traits
monomorphization
dispatch
Send/Sync
atomics
memory ordering
pinning
async runtime mechanics
ABI
FFI
unsafe invariants
OS interfaces
cache behavior
```

So I deliberately put a lot of projects **outside web development**.

---

# What I would change from the obvious roadmap

I initially considered putting:

```text
CLI
→ web server
→ REST API
→ database
→ distributed system
→ compiler
```

as the primary sequence.

I rejected it.

Why?

Because it produces a developer who is good at building applications but can still have huge gaps in:

- memory models
- operating systems
- concurrency
- algorithms
- compilers
- networking
- architecture
- performance
- unsafe Rust
- language design

The revised roadmap intentionally forces those gaps closed.

I also would **not** make you implement everything from scratch.

That sounds “more hardcore,” but it's actually bad engineering.

A professional Rust engineer should know when to use:

```text
Tokio
Serde
Axum
SQLx
Tracing
Tower
Rustls
Rayon
```

and when reinventing something is useful for learning.

The distinction is:

> **Reimplement things to understand them. Use mature implementations to build products.**

---

# The progression I want you to feel

At the beginning:

> “Why won't the compiler let me do this?”

Then:

> “Oh, I understand the ownership problem.”

Then:

> “I can design the ownership so the compiler accepts it.”

Then:

> “I can design the API so invalid ownership states are difficult to express.”

Then:

> “I can make the concurrency model explicit.”

Then:

> “I understand the underlying runtime and OS behavior.”

Then:

> “I can reason about the correctness and performance of the system.”

And eventually:

> **“I can design the system before I write the code.”**

That's the real endpoint.

---

# Recommended order

If I were actually putting you through this curriculum, I'd use this exact sequence:

```text
01  Unit Converter
02  Todo CLI
03  Text Search Engine
04  In-Memory Database
05  Expression Language
06  Generic Collections

07  Multithreaded Web Server
08  Thread Pool
09  Async Runtime
10  Memory Allocator
11  Shell
12  Concurrent Web Crawler

13  Production REST API
14  Authentication Service
15  Job Queue
16  Key-Value Database
17  Reverse Proxy
18  Distributed Cache
19  Observability Platform

20  Filesystem
21  SQLite-like Database
22  Compiler
23  Static Analyzer
24  Language Server
25  WASM Runtime

26  OS Kernel
27  Network Stack
28  Distributed Database / Raft
29  Production Rust Platform
30  Your Own Serious Product
```

And alongside **all 30**, continuously study:

```text
Algorithms
Data Structures
OS
Networking
Databases
Distributed Systems
Computer Architecture
Concurrency
Cryptography
Compilers
Testing
Security
Performance
API Design
DevOps
Git
Linux
```

---

# One final adjustment: don't make this a “30-project checklist”

This roadmap is intentionally not a promise that completing thirty projects automatically makes you an expert.

Expertise is demonstrated by the depth of your reasoning and the systems you can independently design, not by project count.

For the best results, treat each project as a vehicle for progressively deeper mastery, with increasingly strict quality gates for correctness, engineering, performance, security, and operational reliability.
