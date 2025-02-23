use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use cosmic::{
    app,
    iced::{
        widget::text, Element, Subscription, Task, Renderer,
        futures::stream::{self, BoxStream, StreamExt},
    },
};
use cosmic::widget::autosize;

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<StatusApplet>(())
}

pub struct StatusApplet {
    core: cosmic::app::Core,
    latest_output: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateOutput(String),
}

impl cosmic::Application for StatusApplet {
    type Flags = ();
    type Message = Message;
    type Executor = cosmic::SingleThreadExecutor;

    const APP_ID: &'static str = "io.github.theshatterstone.cosmic-mystatus";

    fn init(
        core: app::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::app::Message<Self::Message>>) {
        (
            Self {
                core,
                latest_output: String::from("Loading..."),
            },
            Task::none(),
        )
    }

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(run_i3status)
    }

    fn update(&mut self, message: Message) -> Task<cosmic::app::Message<Self::Message>> {
        match message {
            Message::UpdateOutput(output) => {
                self.latest_output = output;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message, cosmic::Theme, Renderer> {
        let content = text(&self.latest_output).size(16);
        autosize::autosize(content, cosmic::widget::Id::unique()).into()
    }
}

fn run_i3status() -> BoxStream<'static, Message> {
    let config_path = std::env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap()))
        + "/i3status/config";

    Box::pin(stream::unfold(config_path, |config_path| async move {
        let process = Command::new("i3status")
            .arg("-c")
            .arg(&config_path) // Use reference to state variable
            .stdout(Stdio::piped())
            .stderr(Stdio::null())    // Suppress standard error
            .spawn()
            .expect("Failed to start i3status");

        let stdout = process.stdout.expect("Failed to capture i3status output");
        let reader = BufReader::new(stdout);

        let mut lines = reader.lines();
        if let Some(Ok(line)) = lines.next() {
            return Some((Message::UpdateOutput(line), config_path));
        }

        None
    }))
}


