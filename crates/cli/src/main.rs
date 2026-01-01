// SPDX-License-Identifier: Apache-2.0

//! Audio Ninja CLI - Command-line interface for daemon control

mod tui;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde_json::Value;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(name = "audio-ninja")]
#[command(about = "Audio Ninja command-line interface", long_about = None)]
#[command(version)]
struct Args {
    /// Daemon API URL
    #[arg(short, long, default_value = "http://127.0.0.1:8080")]
    daemon: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show daemon status
    Status,

    /// Show daemon information
    Info,

    /// Interactive terminal UI dashboard
    Tui,

    /// Speaker management
    #[command(subcommand)]
    Speaker(SpeakerCommands),

    /// Layout configuration
    #[command(subcommand)]
    Layout(LayoutCommands),

    /// Transport control
    #[command(subcommand)]
    Transport(TransportCommands),

    /// Input device management
    #[command(subcommand)]
    Input(InputCommands),

    /// Output device management
    #[command(subcommand)]
    Output(OutputCommands),

    /// Calibration
    #[command(subcommand)]
    Calibration(CalibrationCommands),

    /// Show statistics
    Stats,
}

#[derive(Subcommand, Debug)]
enum SpeakerCommands {
    /// List all speakers
    List,

    /// Discover speakers on the network
    Discover,

    /// Get information about a specific speaker
    Get {
        /// Speaker ID (UUID)
        id: Uuid,
    },

    /// Remove a speaker
    Remove {
        /// Speaker ID (UUID)
        id: Uuid,
    },

    /// Show speaker statistics
    Stats {
        /// Speaker ID (UUID)
        id: Uuid,
    },
}

#[derive(Subcommand, Debug)]
enum LayoutCommands {
    /// Show current layout
    Get,

    /// Set layout from preset
    Set {
        /// Layout preset (stereo, 5.1, 7.1, etc.)
        preset: String,
    },
}

#[derive(Subcommand, Debug)]
enum TransportCommands {
    /// Start playback
    Play,

    /// Pause playback
    Pause,

    /// Stop playback
    Stop,

    /// Show transport status
    Status,

    /// Load audio file for playback
    LoadFile {
        /// Path to audio file
        file_path: String,
    },

    /// Set transport mode (file/stream/mixed)
    Mode {
        /// Transport mode: file (file playback only), stream (live input only), mixed (both)
        mode: String,
    },
}

#[derive(Subcommand, Debug)]
enum InputCommands {
    /// List all input devices
    List,

    /// Select input source (system audio or external device)
    Select {
        /// Source ID (system or device name)
        source_id: String,
    },

    /// Show current input status
    Status,
}

#[derive(Subcommand, Debug)]
enum OutputCommands {
    /// List all output devices
    List,

    /// Select output device (speaker or headphones)
    Select {
        /// Device ID
        device_id: String,
    },

    /// Show current output status
    Status,
}

#[derive(Subcommand, Debug)]
enum CalibrationCommands {
    /// Start calibration
    Start,

    /// Show calibration status
    Status,

    /// Apply calibration results
    Apply,
}

struct ApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    async fn get(&self, path: &str) -> Result<Value> {
        let url = format!("{}/api/v1{}", self.base_url, path);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        let json = response.json().await.context("Failed to parse JSON")?;
        Ok(json)
    }

    async fn post(&self, path: &str, body: Option<Value>) -> Result<()> {
        let url = format!("{}/api/v1{}", self.base_url, path);
        let mut request = self.client.post(&url);

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await.context("Failed to send request")?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        Ok(())
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}/api/v1{}", self.base_url, path);
        let response = self
            .client
            .delete(&url)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            anyhow::bail!("Request failed with status: {}", response.status());
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = ApiClient::new(args.daemon.clone());

    match args.command {
        Commands::Tui => {
            run_tui(args.daemon).await?;
        }

        Commands::Status => {
            let status = client.get("/status").await?;
            println!("{}", serde_json::to_string_pretty(&status)?);
        }

        Commands::Info => {
            let info = client.get("/info").await?;
            println!("{}", serde_json::to_string_pretty(&info)?);
        }

        Commands::Speaker(cmd) => match cmd {
            SpeakerCommands::List => {
                let speakers = client.get("/speakers").await?;
                println!("{}", serde_json::to_string_pretty(&speakers)?);
            }

            SpeakerCommands::Discover => {
                client.post("/speakers/discover", None).await?;
                println!("Speaker discovery started");
            }

            SpeakerCommands::Get { id } => {
                let speaker = client.get(&format!("/speakers/{}", id)).await?;
                println!("{}", serde_json::to_string_pretty(&speaker)?);
            }

            SpeakerCommands::Remove { id } => {
                client.delete(&format!("/speakers/{}", id)).await?;
                println!("Speaker {} removed", id);
            }

            SpeakerCommands::Stats { id } => {
                let stats = client.get(&format!("/speakers/{}/stats", id)).await?;
                println!("{}", serde_json::to_string_pretty(&stats)?);
            }
        },

        Commands::Layout(cmd) => match cmd {
            LayoutCommands::Get => {
                let layout = client.get("/layout").await?;
                println!("{}", serde_json::to_string_pretty(&layout)?);
            }

            LayoutCommands::Set { preset } => {
                let body = serde_json::json!({ "preset": preset });
                client.post("/layout", Some(body)).await?;
                println!("Layout set to {}", preset);
            }
        },

        Commands::Transport(cmd) => match cmd {
            TransportCommands::Play => {
                client.post("/transport/play", None).await?;
                println!("Playback started");
            }

            TransportCommands::Pause => {
                client.post("/transport/pause", None).await?;
                println!("Playback paused");
            }

            TransportCommands::Stop => {
                client.post("/transport/stop", None).await?;
                println!("Playback stopped");
            }

            TransportCommands::Status => {
                let status = client.get("/transport/status").await?;
                println!("{}", serde_json::to_string_pretty(&status)?);
            }

            TransportCommands::LoadFile { file_path } => {
                let body = serde_json::json!({ "file_path": file_path });
                client.post("/transport/load-file", Some(body)).await?;
                println!("Audio file loaded: {}", file_path);
            }

            TransportCommands::Mode { mode } => {
                let body = serde_json::json!({ "mode": mode });
                client.post("/transport/mode", Some(body)).await?;
                println!("Transport mode set to: {}", mode);
            }
        },

        Commands::Input(cmd) => match cmd {
            InputCommands::List => {
                let devices = client.get("/input/devices").await?;
                println!("{}", serde_json::to_string_pretty(&devices)?);
            }

            InputCommands::Select { source_id } => {
                let body = serde_json::json!({ "source_id": source_id });
                client.post("/input/select", Some(body)).await?;
                println!("Input source selected: {}", source_id);
            }

            InputCommands::Status => {
                let status = client.get("/input/status").await?;
                println!("{}", serde_json::to_string_pretty(&status)?);
            }
        },

        Commands::Output(cmd) => match cmd {
            OutputCommands::List => {
                let devices = client.get("/output/devices").await?;
                println!("{}", serde_json::to_string_pretty(&devices)?);
            }

            OutputCommands::Select { device_id } => {
                let body = serde_json::json!({ "device_id": device_id });
                client.post("/output/select", Some(body)).await?;
                println!("Output device selected: {}", device_id);
            }

            OutputCommands::Status => {
                let status = client.get("/output/status").await?;
                println!("{}", serde_json::to_string_pretty(&status)?);
            }
        },

        Commands::Calibration(cmd) => match cmd {
            CalibrationCommands::Start => {
                client.post("/calibration/start", None).await?;
                println!("Calibration started");
            }

            CalibrationCommands::Status => {
                let status = client.get("/calibration/status").await?;
                println!("{}", serde_json::to_string_pretty(&status)?);
            }

            CalibrationCommands::Apply => {
                client.post("/calibration/apply", None).await?;
                println!("Calibration applied");
            }
        },

        Commands::Stats => {
            let stats = client.get("/stats").await?;
            println!("{}", serde_json::to_string_pretty(&stats)?);
        }
    }

    Ok(())
}

async fn run_tui(base_url: String) -> Result<()> {
    use crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::prelude::*;

    let mut stdout = std::io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let client = ApiClient::new(base_url.clone());
    let mut app = tui::App::new(base_url);

    // Initial data load
    if let Ok(status) = client.get("/status").await {
        app.status = Some(status);
    }
    if let Ok(speakers) = client.get("/speakers").await {
        app.speakers = Some(speakers);
    }
    if let Ok(layout) = client.get("/layout").await {
        app.layout = Some(layout);
    }
    if let Ok(transport) = client.get("/transport/status").await {
        app.transport_status = Some(transport);
    }
    if let Ok(calibration) = client.get("/calibration/status").await {
        app.calibration_status = Some(calibration);
    }
    if let Ok(stats) = client.get("/stats").await {
        app.stats = Some(stats);
    }

    let result = loop {
        terminal.draw(|f| {
            tui::ui::draw(f, &app);
        })?;

        if tui::handler::handle_input(&mut app).await {
            break Ok::<(), anyhow::Error>(());
        }

        if !app.running {
            break Ok(());
        }
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
