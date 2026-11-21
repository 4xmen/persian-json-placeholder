
# راهنمای استقرار Rustavel (Deployment Guide)

محیط Production · Linux · Rust · systemd · Nginx

## ۱. ساختار نهایی سرور

مسیر اصلی استقرار برنامه:

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

مسئولیت دایرکتوری‌ها:

- `rustavel-app`: فایل اجرایی کامپایل‌شده برنامه.
- `.env`: تنظیمات محیط Production.
- `storage/`: فایل‌های تولیدشده توسط برنامه و لاگ‌ها.
- `app/src/resource/`: منابع موردنیاز برنامه.
- `public/`: فایل‌های عمومی و استاتیک.
- `resources/`: منابع جانبی موردنیاز در زمان اجرا.

ساختار منابع باید با مسیرهایی که برنامه در زمان اجرا انتظار دارد مطابقت داشته باشد.

## ۲. آماده‌سازی و Build

نسخه Rust باید با حداقل نسخه اعلام‌شده در پروژه سازگار باشد.

نصب Target:


```bash
rustup target add x86_64-unknown-linux-musl
```

در صورت نیاز، نصب ابزارهای Build در Debian/Ubuntu:


```bash
sudo apt update

sudo apt install -y \
    musl-tools \
    build-essential \
    pkg-config
```

دریافت پروژه و Build:


```bash
git clone https://github.com/4xmen/rustavel.git

cd rustavel

cargo build --release --target x86_64-unknown-linux-musl
```

خروجی در مسیر زیر قرار می‌گیرد:

```text
target/x86_64-unknown-linux-musl/release/
```

فایل اجرایی صحیح را از خروجی Cargo انتخاب کرده و با نام `rustavel-app` برای سرور آماده کنید.

```bash
cp target/x86_64-unknown-linux-musl/release/<binary-name> \
   /opt/rustavel/rustavel-app
```

## ۳. آماده‌سازی دیتابیس

توصیه‌شدهفرآیند آماده‌سازی دیتابیس بهتر است پیش از Deploy و در محیط Local یا محیط مستقل آماده‌سازی انجام شود.

مراحل پیشنهادی:

1. اجرای Migrationهای موردنیاز.
2. اجرای Seedهای موردنیاز.
3. بررسی صحت ساختار دیتابیس و اطلاعات اولیه.
4. تهیه Backup از دیتابیس نهایی.
5. انتقال دیتابیس آماده و سالم به سرور Production.
6. تنظیم اطلاعات اتصال دیتابیس در فایل `.env` سرور.

در این روش نیازی به حضور ابزارهای CLI، Migration و Seed در سرور Production نیست.

همچنین نیازی نیست سورس‌کد کامل پروژه یا ابزارهای توسعه روی سرور قرار بگیرند.

نکته: این رویکرد برای Deployهای معمول و دیتابیس‌های قابل انتقال مناسب است. برای Migrationهای بعدی نیز بهتر است یک فرآیند کنترل‌شده و نسخه‌بندی‌شده داشته باشیم؛ Migrationهای مخرب یا تغییرات ناسازگار با نسخه قبلی باید با برنامه‌ریزی جداگانه اجرا شوند.

## ۴. ایجاد کاربر و دایرکتوری‌ها

ایجاد کاربر اختصاصی:

```bash
adduser --system \
  --group \
  --home /opt/rustavel \
  --shell /usr/sbin/nologin \
  rustavel
```

ایجاد دایرکتوری‌ها:

```bash
mkdir -p /opt/rustavel/{storage,storage/logs,public,resources}
```

تنظیم دسترسی‌ها:

```bash
chown -R rustavel:rustavel /opt/rustavel/storage

chmod -R u+rwX /opt/rustavel/storage

chmod 755 /opt/rustavel
chmod 755 /opt/rustavel/public # copy from /
chmod 755 /opt/rustavel/resources # copy from /app/src/resources
chmod 755 /opt/rustavel/rustavel-app # copy from /
```

فایل `.env`:

```bash
chown root:rustavel /opt/rustavel/.env
chmod 640 /opt/rustavel/.env
```

## ۵. انتقال فایل‌های برنامه

فایل‌ها و دایرکتوری‌های موردنیاز را به سرور منتقل کنید:

- فایل اجرایی `rustavel-app`
- فایل `.env`
- دایرکتوری `public`
- منابع موردنیاز برنامه
- دیتابیس آماده‌شده

در نهایت:

```bash
chmod +x /opt/rustavel/rustavel-app
```

## ۶. تنظیم Environment

فایل تنظیمات Production:

```bash
nano /opt/rustavel/.env
```

مبنای تنظیمات باید فایل `.env.example` پروژه باشد.

مقادیر مهم:


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

سایر تنظیمات موردنیاز برنامه نیز باید از `.env.example` منتقل و متناسب با محیط Production تنظیم شوند.

## ۷. راه‌اندازی systemd

ایجاد فایل سرویس:

```bash
nano /etc/systemd/system/rustavel.service
```

محتوای فایل:


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

فعال‌سازی سرویس:

```bash
systemctl daemon-reload

systemctl enable --now rustavel
```

بررسی وضعیت:

```bash
systemctl status rustavel
```

مشاهده لاگ‌ها:

```bash
journalctl -u rustavel -f
```

## ۸. تنظیم Nginx

در حالت معمول، Nginx درخواست‌ها را به پورت داخلی برنامه منتقل می‌کند.

نمونه تنظیمات:


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

بررسی و بارگذاری مجدد:

```bash
nginx -t

systemctl reload nginx
```

برای محیط Production باید HTTPS نیز فعال باشد.

## ۹. تنظیم HestiaCP

در صورت استفاده از HestiaCP، دو Template اختصاصی ایجاد می‌کنیم:

```text
rustavel.tpl
rustavel.stpl
```

- `rustavel.tpl`: تنظیمات HTTP
- `rustavel.stpl`: تنظیمات HTTPS

محتوای این دو فایل همان تنظیمات Reverse Proxy بخش Nginx است که با متغیرهای اختصاصی HestiaCP مانند `%domain_idn%`، `%ip%` و `%web_port%` هماهنگ شده‌اند.

پس از ایجاد Templateها:

1. در تنظیمات Web Domain، قالب `rustavel` را انتخاب کنید.
2. در صورت نیاز SSL را فعال کنید.
3. قالب SSL مربوطه را انتخاب کنید.
4. تنظیمات دامنه را ذخیره و بازسازی کنید.
5. صحت تنظیمات Nginx را بررسی کنید.

```bash
nginx -t

systemctl reload nginx
```

فایل‌های پیش‌فرض HestiaCP نباید مستقیماً ویرایش شوند.

## ۱۰. بررسی نهایی

پس از Deploy موارد زیر بررسی شوند:

```bash
# وضعیت برنامه
systemctl status rustavel

# پورت برنامه
ss -lntp | grep 3000

# تست داخلی
curl -I http://127.0.0.1:3000

# بررسی Nginx
nginx -t

# تست دامنه
curl -I https://example.com

# مشاهده لاگ‌ها
journalctl -u rustavel -n 100 --no-pager
```

## ۱۱. به‌روزرسانی برنامه

برای انتشار نسخه جدید:

```bash
# توقف برنامه
systemctl stop rustavel

# جایگزینی فایل اجرایی
cp rustavel-app /opt/rustavel/rustavel-app

# تنظیم دسترسی
chmod 755 /opt/rustavel/rustavel-app

# اجرای مجدد
systemctl start rustavel

# بررسی وضعیت
systemctl status rustavel
```

قبل از جایگزینی، بهتر است از Binary قبلی نسخه پشتیبان تهیه شود تا در صورت بروز مشکل امکان Rollback وجود داشته باشد.

## نکات مهم معماری Deploy

- سرور Production باید فقط شامل فایل‌های موردنیاز برای اجرای برنامه باشد.
- سورس‌کد کامل پروژه، ابزارهای توسعه و ابزارهای CLI برای اجرای عادی برنامه ضروری نیستند.
- Migration و Seed ترجیحاً در Local یا محیط آماده‌سازی اجرا شوند.
- دیتابیس آماده و بررسی‌شده به Production منتقل شود.
- دسترسی نوشتن برنامه به Storage محدود باشد.
- اطلاعات حساس فقط در Environment قرار بگیرند.
- اجرای برنامه تحت کاربر اختصاصی انجام شود، نه Root.

## منابع مورد نیاز 

- [Rustavel GitHub Repository](<https://github.com/4xmen/rustavel?utm_source=chatgpt.com>)
- [Environment Example](<https://github.com/4xmen/rustavel/blob/master/.env.example?utm_source=chatgpt.com>)
- [Cargo Workspace](<https://github.com/4xmen/rustavel/blob/master/Cargo.toml?utm_source=chatgpt.com>)
- [HestiaCP Web Templates Documentation](<https://hestiacp.com/docs/server-administration/web-templates.html?utm_source=chatgpt.com>)

