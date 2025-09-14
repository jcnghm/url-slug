# URL Slug

<div align="center">
  <img src="https://raw.githubusercontent.com/rust-lang/rust-artwork/master/logo/rust-logo-256x256.png" alt="Rust Logo" width="100">

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
- **SQLite Database**: Persistent storage with automatic table creation for development or local use
- **Input Validation**: Ensures only valid HTTP/HTTPS URLs are accepted

## Quick Start

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
git clone https://github.com/jcnghm/url-slug.git
cd url-slug

2. Run the application:
cargo run

The server will start on `http://localhost:3000`

## API Endpoints

### Create a Slug

Create a new short URL from a long URL.

**Request:**
POST /shorten
Content-Type: application/json

{
  "url": "https://www.example.com/very/long/path"
}

**Response:**
{
  "slug": "a1B2c3",
  "short_url": "http://localhost:3000/a1B2c3"
}

### Redirect to Original URL

Access a slug to be redirected to the original URL. This increments the click counter.

**Request:**
GET /{slug}

**Response:**
HTTP/1.1 301 Moved Permanently
Location: https://www.example.com/very/long/path

### Get Slug Statistics

Retrieve analytics data for a specific slug.

**Request:**
GET /stats/{slug}

**Response:**
{
  "original_url": "https://www.example.com/very/long/path",
  "slug": "a1B2c3",
  "click_count": 42
}

## Testing

### Using cURL

# Create a slug
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url":"https://github.com"}'

# Test redirect
curl -L http://localhost:3000/YOUR_SLUG

# Check statistics
curl http://localhost:3000/stats/YOUR_SLUG

### Using Postman

1. **Create Slug**: POST to `/shorten` with JSON body containing the URL
2. **Test Redirect**: GET the slug endpoint
3. **View Stats**: GET `/stats/{slug}` to see click analytics

## Dependencies

- **Axum**: Modern, ergonomic web framework
- **SQLx**: Async SQL toolkit with compile-time checked queries
- **Serde**: Serialization framework for JSON handling
- **Tokio**: Asynchronous runtime
- **Rand**: Random number generation for slug creation

## Configuration

The application uses a file-based SQLite database. The connection string in `src/database.rs`:

`SqlitePool::connect("sqlite://slugs.db?mode=rwc")`

## Error Handling

The API returns appropriate HTTP status codes:

- `200 OK`: Successful request
- `301 Moved Permanently`: Redirect response
- `400 Bad Request`: Invalid URL provided
- `404 Not Found`: Slug not found
- `500 Internal Server Error`: Database or server error

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Usage

This project is a proof of concept.
