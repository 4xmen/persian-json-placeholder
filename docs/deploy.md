# Rustavel Deployment Guide

Production Environment · Linux · Rust · systemd · Nginx

## 1. Server Directory Structure

The main deployment directory:

```text
/opt/rustavel/
│
├── rustavel-app
├── .env
│
├── storage/
│   └── logs/
│
├── app/
│   └── src/
│       └── resource/
│
├── public/
│
└── resources/
```

Directory responsibilities:

- `rustavel-app`: Compiled application executable.
- `.env`: Production environment configuration.
- `storage/`: Application-generated files and logs.
- `app/src/resource/`: Application resources.
- `public/`: Public and static files.
- `resources/`: Additional runtime resources.

The resource directory structure must match the paths expected by the application at runtime.

## 2. Build the Application

The Rust toolchain must meet the minimum version declared by the project.

Install the target:

```bash
rustup target add x86_64-unknown-linux-musl
```

Install build dependencies if needed (Debian/Ubuntu):

```bash
sudo apt update

sudo apt install -y \
    musl-tools \
    build-essential \
    pkg-config
```

Clone the repository and build:

```bash
git clone https://github.com/4xmen/rustavel.git

cd rustavel

cargo build --release --target x86_64-unknown-linux-musl
```

Build artifacts are generated in:

```text
target/x86_64-unknown-linux-musl/release/
```

Select the correct executable produced by Cargo and prepare it for deployment under the name `rustavel-app`.

```bash
cp target/x86_64-unknown-linux-musl/release/<binary-name> \
   /opt/rustavel/rustavel-app
```

## 3. Database Preparation

RecommendedDatabase preparation should preferably be performed before deployment, in a local or dedicated staging environment.

Recommended workflow:

1. Run the required migrations.
2. Run the required seeders.
3. Verify the database schema and initial data.
4. Create a backup of the finalized database.
5. Transfer the prepared and verified database to the production server.
6. Configure the production database connection in the server's `.env` file.

With this approach, CLI, Migration and Seed tools are not required on the production server.

The complete source code and development tools are also unnecessary.

Note: This approach is suitable for regular deployments and transferable databases. Future migrations should follow a controlled and versioned process. Destructive migrations or changes incompatible with the previous application version require separate planning.

## 4. Create the Application User and Directories

Create a dedicated system user:

```bash
adduser --system \
  --group \
  --home /opt/rustavel \
  --shell /usr/sbin/nologin \
  rustavel
```

Create directories:

```bash
mkdir -p /opt/rustavel/{storage,storage/logs,public,resources}
```

Set permissions:

```bash
chown -R rustavel:rustavel /opt/rustavel/storage

chmod -R u+rwX /opt/rustavel/storage

chmod 755 /opt/rustavel
chmod 755 /opt/rustavel/public # copy from /
chmod 755 /opt/rustavel/resources # copy from /app/src/resources
chmod 755 /opt/rustavel/rustavel-app # copy from /
```

Configure `.env` permissions:

```bash
chown root:rustavel /opt/rustavel/.env
chmod 640 /opt/rustavel/.env
```

## 5. Upload Application Files

Upload the required files and directories to the server:

- `rustavel-app` executable.
- `.env` file.
- `public` directory.
- Required application resources.
- Prepared database.

Finally:

```bash
chmod +x /opt/rustavel/rustavel-app
```

## 6. Configure Environment Variables

Create the production environment file:

```bash
nano /opt/rustavel/.env
```

Use the project's `.env.example` as the configuration reference.

Important settings:


```dotenv
APP_ENV=production
APP_DEBUG=false

APP_IP=127.0.0.1
APP_PORT=3000

APP_URL=https://example.com

DB_CONNECTION=mysql
DB_HOST=127.0.0.1
DB_PORT=3306
DB_DATABASE=production_db
DB_USERNAME=production_user
DB_PASSWORD=CHANGE_ME
```

All other required application settings must also be copied from `.env.example` and adjusted for the production environment.

## 7. Configure systemd

Create the service file:

```bash
nano /etc/systemd/system/rustavel.service
```

Configuration:

```ini
[Unit]
Description=Rustavel Application
After=network-online.target
Wants=network-online.target

[Service]
Type=simple

User=rustavel
Group=rustavel

WorkingDirectory=/opt/rustavel

ExecStart=/opt/rustavel/rustavel-app

Restart=always
RestartSec=5

EnvironmentFile=/opt/rustavel/.env

NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true

ReadWritePaths=/opt/rustavel/storage

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
systemctl daemon-reload

systemctl enable --now rustavel
```

Check service status:

```bash
systemctl status rustavel
```

View application logs:

```bash
journalctl -u rustavel -f
```

## 8. Configure Nginx

Nginx forwards incoming requests to the application's internal port.

Example configuration:


```nginx
server {
    listen 80;
    server_name example.com;

    location / {
        client_max_body_size 50M;

        proxy_buffering off;
        proxy_request_buffering off;

        proxy_read_timeout 300s;
        proxy_send_timeout 300s;
        proxy_connect_timeout 10s;

        proxy_redirect off;

        proxy_http_version 1.1;

        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";

        proxy_pass http://127.0.0.1:3000;
    }
}
```

Validate and reload Nginx:

```bash
nginx -t

systemctl reload nginx
```

HTTPS must also be enabled for production deployments.

## 9. HestiaCP Configuration

If HestiaCP is used, create two custom templates:

```text
rustavel.tpl
rustavel.stpl
```

- `rustavel.tpl`: HTTP configuration.
- `rustavel.stpl`: HTTPS configuration.

Both templates use the Nginx reverse proxy configuration adapted to HestiaCP-specific variables such as `%domain_idn%`, `%ip%` and `%web_port%`.

After creating the templates:

1. Select `rustavel` as the domain's web template.
2. Enable SSL if required.
3. Select the corresponding SSL template.
4. Save and rebuild the domain configuration.
5. Validate the Nginx configuration.

```bash
nginx -t

systemctl reload nginx
```

Default HestiaCP templates must never be modified directly.

## 10. Final Verification

After deployment, verify the following:

```bash
# Application status
systemctl status rustavel

# Application port
ss -lntp | grep 3000

# Local application test
curl -I http://127.0.0.1:3000

# Nginx configuration
nginx -t

# Domain test
curl -I https://example.com

# Application logs
journalctl -u rustavel -n 100 --no-pager
```

## 11. Application Updates

To deploy a new version:

```bash
# Stop the application
systemctl stop rustavel

# Replace the executable
cp rustavel-app /opt/rustavel/rustavel-app

# Set permissions
chmod 755 /opt/rustavel/rustavel-app

# Start the application
systemctl start rustavel

# Verify status
systemctl status rustavel
```

Always keep a backup of the previous executable before replacing it, allowing rollback if the new version fails.

## Deployment Architecture Notes

- The production server should contain only the files required to run the application.
- Full source code, development tools and CLI utilities are not required for normal application execution.
- Migrations and seeders should preferably run locally or in a dedicated preparation environment.
- The prepared and verified database should be transferred to production.
- Application write permissions should be restricted to storage directories.
- Sensitive configuration must remain in environment variables.
- The application must run under a dedicated system user, never as root.

## References

- [Rustavel GitHub Repository](<https://github.com/4xmen/rustavel?utm_source=chatgpt.com>)
- [Environment Example](<https://github.com/4xmen/rustavel/blob/master/.env.example?utm_source=chatgpt.com>)
- [Cargo Workspace](<https://github.com/4xmen/rustavel/blob/master/Cargo.toml?utm_source=chatgpt.com>)
- [HestiaCP Web Templates Documentation](<https://hestiacp.com/docs/server-administration/web-templates.html?utm_source=chatgpt.com>)

