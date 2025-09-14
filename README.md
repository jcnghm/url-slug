# URL Slug

<div align="center">
  <img src="https://raw.githubusercontent.com/rust-lang/rustacean.net/master/assets/rustacean-flat-noshadow.svg" width="150" alt="Axum Logo">
</div>

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-0.7-blue?logo=rust&logoColor=white)
![SQLx](https://img.shields.io/badge/SQLx-0.7-green?logo=sqlite&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-yellow)

</div>

A URL shortening service built with Rust and Axum. Transform long URLs into short, shareable slugs with built-in click tracking and analytics.

## Features

- **Fast URL Shortening**: Convert long URLs into 6-character slugs
- **Click Tracking**: Monitor how many times each slug is accessed
- **Statistics API**: Get detailed analytics for any slug
- **SQLite Database**: Persistent storage with automatic table creation
- **Input Validation**: Ensures only valid HTTP/HTTPS URLs are accepted
- **RESTful API**: Clean and intuitive endpoint design

## Quick Start

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/url-slug.git
cd url-slug