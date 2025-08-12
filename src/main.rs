use resend_rs::{Resend, Result, types::CreateEmailBaseOptions};
use std::{env, fmt::Display, fs, path::PathBuf};

use dotenv::dotenv;
use serde::Deserialize;
use std::io::ErrorKind;

#[derive(Deserialize)]
enum Urgency {
    #[serde(rename(deserialize = "normal"))]
    Normal,
    #[serde(rename(deserialize = "important"))]
    Important,
}

impl Display for Urgency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Urgency::Normal => write!(f, "Normal"),
            Urgency::Important => write!(f, "Important"),
        }
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Event {
    id: u16,
    year: u16,
    month: u16,
    day: u16,
    event_name: String,
    not_used_int: String,
    not_used_string: String,
    urgency: Urgency,
}

fn read_csv_events() -> Result<Vec<Event>, csv::Error> {
    let mut reader = csv::Reader::from_path("./events.csv").unwrap();

    reader.deserialize().collect()
}

fn create_file() -> std::io::Result<()> {
    let csv_headers: &str = "id,year,month,day,event_name,not_used_int,not_used_string,urgency\n";
    let csv_headers_bytes: &[u8] = csv_headers.as_bytes();

    let home_dir = dirs::home_dir()
        .ok_or_else(|| std::io::Error::new(ErrorKind::NotFound, "Home directory not found"))?;

    let file_location: PathBuf = home_dir.join(".config/calcure/events.csv");
    let destination_file: &str = "./events.csv";

    let bytes_read = fs::read(&file_location)?;

    let mut full_content = Vec::with_capacity(csv_headers_bytes.len() + bytes_read.len());
    full_content.extend_from_slice(csv_headers_bytes);
    full_content.extend_from_slice(&bytes_read);

    fs::write(destination_file, full_content).unwrap();

    Ok(())
}

fn create_html(vec_of_events: Vec<Event>) -> String {
    let mut owned_html: String = r#"
	<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Eingehende Ereignisse</title>
    <style>
        body {
            font-family: 'Helvetica Neue', Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 650px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f9f9f9;
        }
        .header {
            text-align: center;
            margin-bottom: 30px;
            padding-bottom: 20px;
            border-bottom: 1px solid #eaeaea;
        }
        .header h1 {
            font-weight: 300;
            color: #2c3e50;
            margin-bottom: 5px;
        }
        .header p {
            color: #7f8c8d;
            margin-top: 0;
        }
        .event {
            background: white;
            padding: 20px;
            margin-bottom: 15px;
            border-radius: 4px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.05);
            transition: all 0.3s ease;
        }
        .event:hover {
            box-shadow: 0 5px 15px rgba(0,0,0,0.1);
        }
        .event-date {
            font-size: 14px;
            color: #7f8c8d;
            margin-bottom: 5px;
        }
        .event-name {
            font-size: 18px;
            font-weight: 500;
            color: #2c3e50;
            margin: 0 0 10px 0;
        }
        .event-urgency {
            display: inline-block;
            padding: 3px 8px;
            font-size: 12px;
            border-radius: 3px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        .normal {
            background-color: #ecf0f1;
            color: #7f8c8d;
        }
        .important {
            background-color: #ffeaa7;
            color: #e17055;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>Kommende Veranstaltungen</h1>
        <p>Dies sind Ihre wichtigsten Veranstaltungen für heute</p>
    </div>
    "#
    .to_owned();

    for event in vec_of_events {
        let (urgency_class, urgency_text) = match event.urgency {
            Urgency::Normal => ("normal", "normal"),
            Urgency::Important => ("important", "Important"),
        };

        let html_string = format!(
            r#"<div class="event">
        <div class="event-date">{}/{}/{}</div>
        <h2 class="event-name">{}</h2>
        <span class="event-urgency {}">{}</span>
    </div>"#,
            event.day, event.month, event.year, event.event_name, urgency_class, urgency_text
        );

        owned_html.push_str(&html_string);
    }

    owned_html.push_str(
        r#"
    </body>
    </html>
    "#,
    );

    owned_html
}

async fn send_email(html_content: String) -> Result<()> {
    let api_key = env::var("API_KEY").unwrap();
    let resend = Resend::new(&api_key);

    let from = env::var("FROM_RESEND_EMAIL").unwrap();
    let from = &from;

    let to = env::var("RECEIVER_EMAIL").unwrap();
    let to = [&to];
    let subject = "Daily reminder";

    let email = CreateEmailBaseOptions::new(from, to, subject).with_html(&html_content);

    let _email = resend.emails.send(email).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), resend_rs::Error> {
    dotenv().ok();

    create_file().unwrap();
    let events = read_csv_events().unwrap();
    let html_content = create_html(events);
    send_email(html_content).await.unwrap();

    Ok(())
}
