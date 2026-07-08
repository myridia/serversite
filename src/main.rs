use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use tera::{Context, Tera};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    title: String,
    description: String,
    updated: String,
    mail: Mail,
    services: Vec<Service>,
    impressum: Impressum,
    contact_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mail {
    host: String,
    smtp: String,
    imap: String,
    pop3: String,
    security: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Service {
    name: String,
    icon: String,
    description: String,
    links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Link {
    label: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Impressum {
    name: String,
    email: String,
    url: String,
    address: Vec<String>,
    tel: String,
    privacy: String,
}

fn default_config() -> Config {
    Config {
        title: "web.salamander-jewelry.net".into(),
        description: "Internal server overview".into(),
        updated: chrono_now(),
        mail: Mail {
            host: "web.salamander-jewelry.net".into(),
            smtp: "25 / 26 / 587".into(),
            imap: "143".into(),
            pop3: "110".into(),
            security: "StartTLS".into(),
        },
        services: vec![
            Service {
                name: "Froxlor".into(),
                icon: "&#x1f4e1;".into(),
                description: "Server control panel".into(),
                links: vec![Link { label: "Open".into(), url: "../froxlor".into() }],
            },
            Service {
                name: "phpMyAdmin".into(),
                icon: "&#x1f4c2;".into(),
                description: "MySQL / MariaDB administration".into(),
                links: vec![Link { label: "Open".into(), url: "../phpmyadmin".into() }],
            },
        ],
        impressum: Impressum {
            name: "Myridia Co., Ltd.".into(),
            email: "info@myridia.com".into(),
            url: "https://www.myridia.com".into(),
            address: vec![
                "Room: 82/090, Floor: 5, Building B".into(),
                "83/198 Salaya".into(),
                "Phutthamonthon, Nakhon Pathom 73170".into(),
                "Thailand".into(),
            ],
            tel: "+66832163880".into(),
            privacy: "This site respects your privacy.".into(),
        },
        contact_text: "Contact us at info@myridia.com".into(),
    }
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    // days since epoch, then year/month/day
    let days = secs / 86400;
    let mut y = 1970i64;
    let mut d = days as i64;
    loop {
        let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
        let dim = 365 + if leap { 1 } else { 0 };
        if d < dim { break; }
        d -= dim;
        y += 1;
    }
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let mdays = [31, if leap {29} else {28}, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut m = 0;
    while d >= mdays[m] { d -= mdays[m]; m += 1; }
    format!("{:04}-{:02}-{:02}", y, m + 1, d as u64 + 1)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("config.json"));

    let cfg: Config = match fs::read_to_string(&config_path) {
        Ok(content) => serde_json::from_str(&content)?,
        Err(_) => {
            let cfg = default_config();
            let json = serde_json::to_string_pretty(&cfg)?;
            fs::write(&config_path, &json)?;
            println!("Created default {}", config_path.display());
            cfg
        }
    };

    let mut tera = Tera::default();
    tera.add_raw_template("index.html.tera", include_str!("../templates/index.html.tera"))?;

    let mut ctx = Context::new();
    ctx.insert("cfg", &cfg);

    let html = tera.render("index.html.tera", &ctx)?;
    fs::create_dir_all("public")?;
    fs::write("public/index.html", html)?;

    fs::write(
        "index.html",
        "<!DOCTYPE html>\n<html><head><meta http-equiv=\"refresh\" content=\"0;url=public/index.html\"><title>Redirect</title></head><body><a href=\"public/index.html\">public/index.html</a></body></html>\n",
    )?;

    println!("Generated public/index.html and index.html from {}", config_path.display());
    Ok(())
}
