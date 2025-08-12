# Calcxporte_r

`calcxporte_r` is a Rust command-line tool that integrates with the [`calcure`](https://github.com/anufrievroman/calcure) TUI calendar. It reads your scheduled events from `calcure`'s data file, formats them into a styled HTML email, and sends you a daily reminder using the Resend email API.

## How It Works

The application executes the following steps:
1.  Loads environment variables (API Key, recipient emails) from a `.env` file.
2.  Locates the `calcure` events file at `~/.config/calcure/events.csv`.
3.  Creates a temporary copy of this file named `events.csv` in the project's root directory. It prepends the necessary CSV headers (`id,year,month,day,...`) to this copy to enable correct parsing, without modifying your original `calcure` file.
4.  Parses the event data from the temporary CSV file.
5.  Generates a clean, styled HTML body that lists each event with its date and urgency level. Events marked as `important` are highlighted differently from those marked `normal`.
6.  Uses the Resend API to send the generated HTML as an email to your configured recipient address.

## Prerequisites

Before you begin, ensure you have the following:
*   **Rust**: Version 1.65 or higher.
*   **Calcure**: A working installation of `calcure` with events stored in `~/.config/calcure/events.csv`.
*   **Resend Account**: A Resend account with a generated API key and a configured sending domain/email address.

## Setup and Configuration

1.  **Clone the Repository**
    ```bash
    git clone https://github.com/sponkurtus2/calcxporte_r.git
    cd calcxporte_r
    ```

2.  **Configure Environment Variables**
    Create a `.env` file in the project root by copying the example file:
    ```bash
    cp .env.example .env
    ```
    Open the `.env` file and fill in your details:
    ```ini
    # Your secret API key from Resend.
    API_KEY=YOUR_RESEND_API_KEY
    
    # The email address that will receive the reminder.
    RECEIVER_EMAIL=your.email@example.com
    
    # The "from" email address configured in your Resend account.
    FROM_RESEND_EMAIL=sender@your-resend-domain.com
    ```

## Usage

To run the application and send a reminder email, execute the following command from the project's root directory:
```bash
cargo run --release
```
The program will read your `calcure` events and send the email immediately.

## Automating Daily Reminders

For automatic daily reminders, you can schedule the application to run using a cron job.

1.  First, build the optimized release binary:
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/calcxporteR`.

2.  Open your crontab for editing:
    ```bash
    crontab -e
    ```

3.  Add a new line to schedule the job. The following example runs the application every day at 8:00 AM. Make sure to replace `/path/to/your/project` with the absolute path to the `calcxporte_r` directory.
    ```crontab
    # Send calcure email reminder every day at 8:00 AM
    0 8 * * * cd /path/to/your/project/calcxporte_r && ./target/release/calcxporteR
