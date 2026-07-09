# serversite

A lightweight, configuration-driven static HTML website generator for presenting servers and services. Built in Rust, serversite generates a modern, responsive dashboard from a simple JSON configuration file.

## Features

- 🚀 **Fast & Lightweight** - Written in Rust for excellent performance
- ⚙️ **Configuration-Driven** - Simple JSON config, no templating language needed
- 📱 **Responsive Design** - Mobile-friendly UI built with Pico CSS
- 🌓 **Dark/Light Mode** - Automatic theme switching with localStorage persistence
- 🔗 **Link Management** - Easily organize and display multiple services/websites
- 📦 **Self-Contained** - Single binary with no external dependencies

## How It Works

`serversite-gen` is a static site generator that:

1. Reads your `config.json` file containing site structure, services, and metadata
2. Uses Tera templating to render an HTML page from your configuration
3. Generates `public/index.html` - your ready-to-serve static site
4. Creates a redirect at `index.html` for easy access

The generated site is a beautiful, modern dashboard that displays your configured services and links in an organized, visually appealing layout.

## Quick Start

### Using Pre-built Binaries

Download the latest binary for your operating system:

- **Linux**: [serversite-gen (Linux)](https://github.com/myridia/serversite/releases/download/main/serversite-gen)
- **FreeBSD**: [serversite-gen_freebsd (FreeBSD)](https://github.com/myridia/serversite/releases/download/main/serversite-gen_freebsd)

```bash
# Download for Linux
wget https://github.com/myridia/serversite/releases/download/main/serversite-gen
chmod +x serversite-gen

# Or for FreeBSD
wget https://github.com/myridia/serversite/releases/download/main/serversite-gen_freebsd
chmod +x serversite-gen_freebsd
```

### Running the Generator

```bash
# Run with default config location (./config.json)
./serversite-gen

# Or specify a custom config path
./serversite-gen /path/to/custom-config.json
```

This will:
- Create a `public/` directory
- Generate `public/index.html` from your configuration
- Create a redirect at `index.html`

### Configuration

Edit `config.json` to customize your site:

```json
{
  "header": {
    "title": "ServerPanel",
    "tagline": "Infrastructure Management",
    "description": "Centralized server monitoring and management platform",
    "nav": [
      { "label": "Dashboard", "url": "/" },
      { "label": "Servers", "url": "/servers" },
      { "label": "Services", "url": "/services" },
      { "label": "Monitor", "url": "/monitor" }
    ]
  },
  "Websites": [
    {
      "name": "GMS",
      "icon": "&#x1f4e1;",
      "description": "Server control panel with real-time metrics.",
      "link": "https://gms.example.net"
    },
    {
      "name": "Mail",
      "icon": "&#x2709;",
      "description": "Mail server administration.",
      "link": "https://mail.example.net"
    }
  ],
  "contact": {
    "text": "Reach out to our support team anytime.",
    "email": "support@serverpanel.example"
  },
  "footer": {
    "updated": "2026-07-09"
  }
}
```

**Configuration Fields:**

- `header.title` - Main site title
- `header.tagline` - Short tagline/slogan
- `header.description` - Long description
- `header.nav` - Navigation menu items (label + url)
- `Websites` - Array of services to display (name, icon, description, link)
- `contact.text` - Support text
- `contact.email` - Contact email address
- `footer.updated` - Last update date

### Building from Source

Requirements:
- Rust 1.70+
- Cargo

```bash
# Development (with live reload)
make dev
# or: cargo watch -x run -w src -w Cargo.toml -w config.json

# Release build
make release
# or: cargo build --release
```

The compiled binary will be in `target/release/serversite-gen`.

## Serving Your Site

Once generated, serve the `public/` directory with any web server:

```bash
# Using Python
python3 -m http.server --directory public

# Using Node.js (http-server)
npx http-server public

# Using Nginx
# Point root to your public/ directory
```

## UI Features

The generated site includes:

- **Sticky Header** - Navigation stays visible while scrolling
- **Theme Toggle** - Light/Dark/Auto mode selector (stores preference locally)
- **Mobile Navigation** - Hamburger menu for small screens
- **Service Grid** - Responsive card layout for your services
- **Contact Section** - Email link to support team
- **Dynamic Footer** - Links and metadata organized in columns

## Development

### Project Structure

```
serversite/
├── src/
│   └── main.rs           # Generator logic
├── templates/
│   └── index.html.tera   # HTML template
├── config.json           # Site configuration
├── Cargo.toml            # Rust dependencies
├── Makefile              # Build shortcuts
├── public/               # Generated output
│   └── index.html
└── README.md
```

### Dependencies

- `serde` & `serde_json` - JSON parsing
- `tera` - Template rendering

## License

Licensed under the GNU General Public License v3.0 - see [LICENSE](LICENSE) for details.

---

## Additional Resources

### Making phpMyAdmin Accessible via Froxlor on a Custom Domain

1. In Froxlor, go to **Domains → Edit your domain** (e.g. `gms.salamander-jewelry.net`)
2. Click the **SSL** tab
3. Scroll down to **IPs and Ports**, then click the IP/port entry
4. Scroll to **Own SSL vHost-settings**

> The content of this field will be included into this ip/port vHost container directly. You can use the following variables:
> `{DOMAIN}`, `{DOCROOT}`, `{CUSTOMER}`, `{IP}`, `{PORT}`, `{SCHEME}`, `{FPMSOCKET}` (if applicable)
>
> **Attention:** The code won't be checked for any errors. If it contains errors, webserver might not start again!

5. Paste the following (Nginx example):

```
location /phpmyadmin {
    alias /usr/share/phpmyadmin/;
    index index.php;

    location ~ \.php$ {
        fastcgi_split_path_info ^(.+?\.php)(/.*)$;
        include /etc/nginx/fastcgi_params;
        fastcgi_param SCRIPT_FILENAME $request_filename;
        try_files $fastcgi_script_name =404;
        set $path_info $fastcgi_path_info;
        fastcgi_param PATH_INFO $path_info;
        fastcgi_pass unix:/var/run/php/1-froxlor.panel-web.salamander-jewelry.net-php-fpm.socket;
        fastcgi_index index.php;
    }

    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff2?|ttf|eot)$ {
        expires max;
        log_not_found off;
    }
}
```

6. Save — the domain will now serve phpMyAdmin at `https://your-domain/phpmyadmin/`.
