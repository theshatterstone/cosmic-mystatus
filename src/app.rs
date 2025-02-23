use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use cosmic::{
    app,
    iced::{
        widget::text, Alignment, Element, Subscription,
        futures::stream::{self, BoxStream, StreamExt},
    },
};
use cosmic::widget::autosize;

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<StatusApplet>(())
}

struct StatusApplet {
    core: cosmic::app::Core,
    latest_output: String,
}

impl StatusApplet {
    fn update_output(&mut self, output: String) {
        self.latest_output = output;
    }
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
    ) -> (Self, cosmic::iced::Task<app::Message<Self::Message>>) {
        (
            Self {
                core,
                latest_output: String::from("Loading..."),
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
        Subscription::from_recipe(I3StatusReader)
    }

    fn update(&mut self, message: Message) -> cosmic::iced::Task<app::Message<Self::Message>> {
        match message {
            Message::UpdateOutput(output) => {
                self.update_output(output);
            }
        }
        cosmic::iced::Task::none()
    }

    fn view(&self) -> Element<Message> {
        let content = text(&self.latest_output).size(16).horizontal_alignment(cosmic::iced::alignment::Horizontal::Center);

        autosize::autosize(content, cosmic::widget::Id::unique()).into()
    }
}

struct I3StatusReader;

impl<H, I> cosmic::iced::subscription::Recipe<H, I> for I3StatusReader
where
    H: std::hash::Hasher,
{
    type Output = Message;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        "i3status_reader".hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: std::sync::Arc<I>,
    ) -> BoxStream<'static, Self::Output> {
        Box::pin(stream::unfold((), move |_| async {
            let output = read_i3status().await;
            Some((Message::UpdateOutput(output), ()))
        }))
    }
}

async fn read_i3status() -> String {
    let process = Command::new("i3status")
        .arg("-c")
        .arg("/home/aleks/.config/i3status/config") // Change this to your config path
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start i3status");

    let stdout = process.stdout.expect("Failed to capture i3status output");
    let reader = BufReader::new(stdout);

    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        return line;
    }

    String::from("No output from i3status")
}
