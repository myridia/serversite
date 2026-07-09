use std::env;
use std::fs;
use std::path::PathBuf;
use tera::{Context, Tera};

const DEFAULT_CONFIG: &str = r##"{
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
    },
    {
      "name": "Cloud",
      "icon": "&#x2601;",
      "description": "Cloud storage and file sync.",
      "link": "https://cloud.example.net"
    },
    {
      "name": "GitLab",
      "icon": "&#x1f5c4;",
      "description": "Self-hosted Git repository management.",
      "link": "https://git.example.net"
    },
    {
      "name": "Wiki",
      "icon": "&#x1f4d6;",
      "description": "Internal knowledge base.",
      "link": "https://wiki.example.net"
    }
  ],
  "contact": {
    "text": "Reach out to our support team anytime.",
    "email": "support@serverpanel.example"
  },
  "footer": {
    "updated": "2026-07-09"
  }
}"##;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("config.json"));

    let (cfg, source) = match fs::read_to_string(&config_path) {
        Ok(raw) => {
            let val: serde_json::Value = serde_json::from_str(&raw)?;
            (val, config_path.display().to_string())
        }
        Err(_) => {
            let val: serde_json::Value = serde_json::from_str(DEFAULT_CONFIG)?;
            (val, "built-in default".to_string())
        }
    };

    let mut tera = Tera::default();
    tera.add_raw_template(
        "index.html.tera",
        include_str!("../templates/index.html.tera"),
    )?;

    let mut ctx = Context::new();
    ctx.insert("cfg", &cfg);

    let html = tera.render("index.html.tera", &ctx)?;
    fs::create_dir_all("public")?;
    fs::write("public/index.html", html)?;
    fs::write(
        "index.html",
        "<!DOCTYPE html>\n<html><head><meta http-equiv=\"refresh\" content=\"0;url=public/index.html\"><title>Redirect</title></head><body><a href=\"public/index.html\">public/index.html</a></body></html>\n",
    )?;

    println!("Generated public/index.html from {}", source);
    Ok(())
}
