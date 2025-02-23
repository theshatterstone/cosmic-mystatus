// SPDX-License-Identifier: GPL-3.0-only

use cosmic::app::{Core, Task};
use cosmic::iced::window::Id;
use cosmic::iced::Limits;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::widget::{self, settings};
use cosmic::{Application, Element};
use std::process::Command;
use tokio::time::{interval, Duration};

#[derive(Default)]
pub struct YourApp {
    core: Core,
    popup: Option<Id>,
    example_row: bool,
    command_output: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    ToggleExampleRow(bool),
    UpdateOutput(String),
}

impl Application for YourApp {
    type Executor = cosmic::executor::Default;
    type Flags = (); 
    type Message = Message;
    const APP_ID: &'static str = "com.example.CosmicAppletTemplate";

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

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}
