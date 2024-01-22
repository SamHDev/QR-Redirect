# QR Configuration Service

A really simple binary to allow dynamic QR redirects for a single URL.

![img.png](.img/img.png)


### Usage

- Users are redirected when visiting the `/` endpoint
- Configuration is possible via the `/configure/` endpoint.
  - Requires Basic Auth
  - URL can be changed at a click of a button
  - Custom URL can be set by *double-clicking* the `Custom URL` button.

### Setup
##### Docker Compose
An example configuration for a docker-compose instance
```yml
services:
  qr:
    image: registry.samhdev.net:443/samhdev/qr-service:v0-0-1
    volumes:
      - type: bind
        source: ./config.toml
        target: /config.toml
        read_only: true
    ports:
      - 80:8080
```
```
docker compose up -d
```
##### Example Configuration
```toml
# Allow Custom URLs
allow_custom = true

# --- define users ---
[[users]]
username = "user" # username
hash = "c0a59647cd759e746d01a1be885694053bc33ee57d82c88382fd8f78bc3ae636" # password as a hex encoded sha256 digest


[[items]]
label = "Item 1" # display label
url = "https://example.org/item1" # redirect url
```

