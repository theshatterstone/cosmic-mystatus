use std::process::Command;
use std::time::Duration;

use cosmic::{
    app,
    iced::{widget::text, Alignment, Subscription},
    widget::autosize,
    Element,
};

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<StatusApplet>(())
}

pub struct StatusApplet {
    core: cosmic::app::Core,
    output: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    UpdateOutput(String),
}

impl StatusApplet {
    fn update_status(&mut self) {
        if let Ok(output) = Command::new("i3status").arg("--run").output() {
            self.output = String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }
}

impl cosmic::Application for StatusApplet {
    type Flags = (); 
    type Message = Message;
    type Executor = cosmic::SingleThreadExecutor;

    const APP_ID: &'static str = "io.github.yourname.status-applet";

    fn init(
        core: app::Core,
        _flags: Self::Flags,
    ) -> (Self, cosmic::iced::Task<app::Message<Self::Message>>) {
        println!("Applet initialized");
        (
            Self {
                core,
                output: "Starting...".to_string(),
            },
            cosmic::iced::Task::none(),
        )
    }

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn subscription(&self) -> Subscription<Message> {
        cosmic::iced::time::every(Duration::from_secs(2)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> cosmic::iced::Task<app::Message<Self::Message>> {
        match message {
            Message::Tick => {
                self.update_status();
            }
            Message::UpdateOutput(new_output) => {
                self.output = new_output;
            }
        }
        cosmic::iced::Task::none()
    }

    fn view(&self) -> Element<Message> {
        let content = text(&self.output).size(16);
        autosize::autosize(content, cosmic::widget::Id::unique()).into()
    }
}
