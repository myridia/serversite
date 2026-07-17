use std::env;
use std::fs;
use std::path::PathBuf;
use tera::{Context, Tera};

const COUCHDB_URL: &str = "https://cb.neriene.com";

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
    let args: Vec<String> = env::args().collect();

    let online_user = args.iter().position(|a| a == "-o").and_then(|i| args.get(i + 1));
    let doc_name = args.iter().position(|a| a == "-d").and_then(|i| args.get(i + 1));

    let config_path = if let Some(user) = online_user {
        let hex: String = user.bytes().map(|b| format!("{:02x}", b)).collect();
        let db = format!("userdb-{}", hex);
        let default_doc = "config".to_string();
        let doc = doc_name.unwrap_or(&default_doc);
        let url = format!("{}/{}/{}", COUCHDB_URL, db, doc);

        println!("Fetching document '{}' from: {}", doc, url);

        match ureq::get(&url).call() {
            Ok(response) => {
                let mut config: serde_json::Value = response.into_json()?;

                if let Some(obj) = config.as_object_mut() {
                    obj.retain(|k, _| !k.starts_with('_'));
                }

                let path = PathBuf::from("config.json");
                fs::write(&path, serde_json::to_string_pretty(&config)?)?;
                println!("Saved config to: {}", path.display());
                path
            }
            Err(e) => {
                println!("Online fetch failed ({}), using default config", e);
                PathBuf::from("config.json")
            }
        }
    } else {
        env::args()
            .nth(1)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("config.json"))
    };

    let (cfg, source) = match fs::read_to_string(&config_path) {
        Ok(raw) => {
            let val: serde_json::Value = serde_json::from_str(&raw)?;
            (val, config_path.display().to_string())
        }
        Err(_) => {
            let val: serde_json::Value = serde_json::from_str(DEFAULT_CONFIG)?;
            fs::write(&config_path, DEFAULT_CONFIG)?;
            println!("Created default config: {}", config_path.display());
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
