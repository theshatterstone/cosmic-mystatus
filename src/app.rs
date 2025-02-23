use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use cosmic::{
    app,
    iced::{
        widget::text, Element, Subscription, Task,
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

    const APP_ID: &'static str = "io.github.yourname.cosmic-status-applet";

    fn init(
        core: app::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<app::Message<Self::Message>>) {
        (
            Self {
                core,
                latest_output: String::from("Loading..."),
            },
            Task::perform(run_i3status(), Message::UpdateOutput),
        )
    }

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::from_stream(run_i3status())
    }

    fn update(&mut self, message: Message) -> Task<app::Message<Self::Message>> {
        match message {
            Message::UpdateOutput(output) => {
                self.latest_output = output;
            }
        }
        Task::none()
    }

    fn view(&self) -> cosmic::iced_core::Element<'_, Self::Message, cosmic::Theme, cosmic::iced_tiny_skia::Renderer> {
        let content = text(&self.latest_output).size(16);
        autosize::autosize(content, cosmic::widget::Id::unique()).into()
    }
}

fn run_i3status() -> BoxStream<'static, String> {
    Box::pin(stream::unfold((), move |_| async {
        let process = Command::new("i3status")
            .arg("-c")
            .arg("/home/aleks/.config/i3status/config") // Custom config path
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start i3status");

        let stdout = process.stdout.expect("Failed to capture i3status output");
        let reader = BufReader::new(stdout);

        let mut lines = reader.lines();
        while let Some(Ok(line)) = lines.next() {
            return Some((line, ()));
        }

        None
    }))
}
