use cosmic::app::{Core, Task};
use cosmic::iced::{Element, Subscription};
use cosmic::iced::widget::text;
use cosmic::Theme;
use std::process::Command;
use std::time::Duration;

#[derive(Default)]
pub struct YourApp {
    core: Core,
    command_output: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

impl YourApp {
    fn update_output(&mut self) {
        let output = Command::new("i3status")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_else(|| "Error".to_string());

        self.command_output = output.trim().to_string();
    }
}

impl cosmic::Application for YourApp {
    type Flags = ();
    type Message = Message;
    type Executor = cosmic::SingleThreadExecutor;

    const APP_ID: &'static str = "com.example.CosmicApplet";

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut app = YourApp {
            core,
            command_output: "Fetching...".to_string(),
        };
        app.update_output(); // Initial command run
        (app, Task::none())
    }

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn subscription(&self) -> Subscription<Message> {
        cosmic::iced::time::every(Duration::from_secs(5)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> Task<Self::Message> {
        match message {
            Message::Tick => {
                self.update_output();
            }
        }
        Task::none()
    }

    fn view(&self) -> cosmic::iced_core::Element<'_, Self::Message, cosmic::Theme, iced_tiny_skia::Renderer> {
        text(&self.command_output).into()
    }
}
