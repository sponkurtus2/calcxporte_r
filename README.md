# Calcure Email Reminder

## Overview
This Rust application integrates with the `calcure` TUI calendar tool to send daily email reminders for events listed in the `events.csv` file. It reads events, generates an HTML email with styled event details, and sends it using the Resend email API.

## Features
- Reads events from `~/.config/calcure/events.csv`.
- Generates a styled HTML email listing events with their dates and urgency levels.
- Sends the email via the Resend API.
- Supports two urgency levels: `Normal` and `Important`.
- Configurable via environment variables.

## Prerequisites
- Rust (stable, version 1.65 or higher recommended).
- Access to the Resend email API with a valid API key.
- A `calcure` installation with an `events.csv` file located at `~/.config/calcure/events.csv`.
- The `events.csv` must have the following columns: `id`, `year`, `month`, `day`, `event_name`, `not_used_int`, `not_used_string`, `urgency`.

## Installation
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <repository-directory>
