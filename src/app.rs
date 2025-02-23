use cosmic::app::{Core, Task};
use cosmic::iced::subscription;
use cosmic::iced::time::{Duration, Instant};
use cosmic::iced::Subscription;
use cosmic::widget;
use cosmic::{Application, Element};
use std::process::Command;

/// Struct representing the applet
#[derive(Default)]
pub struct YourApp {
    core: Core,
    command_output: String,
}

/// Messages for event handling
#[derive(Debug, Clone)]
pub enum Message {
    UpdateOutput(String),
    Tick(Instant),
}

impl Application for YourApp {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.example.CosmicCommandApplet";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = YourApp {
            core,
            command_output: "Fetching...".to_string(),
        };

        (app, Task::none())
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::interval(Duration::from_secs(5)).map(Message::Tick)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Tick(_) => {
                let output = Command::new("i3status") // Change this to your actual command
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .unwrap_or_else(|| "Error".to_string());

                return Task::message(Message::UpdateOutput(output.trim().to_string()));
            }
            Message::UpdateOutput(output) => {
                self.command_output = output;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Self::Message> {
        widget::text(&self.command_output).into()
    }
}
