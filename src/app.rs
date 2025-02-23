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
        let mut app = YourApp {
            core,
            command_output: "Fetching...".to_string(),
            ..Default::default()
        };

        let handle = app.core.applet.handle().clone();
        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(5));
            loop {
                timer.tick().await;
                let output = Command::new("i3status")
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .unwrap_or_else(|| "Error".to_string());

                handle.update_ui(move |app, _| {
                    app.command_output = output.trim().to_string();
                });
            }
        });

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<Self::Message> {
        self.core
            .applet
            .icon_button("display-symbolic")
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<Self::Message> {
        let content_list = widget::list_column()
            .padding(5)
            .spacing(0)
            .add(settings::item(
                "Command Output:",
                widget::text(&self.command_output),
            ));

        self.core.applet.popup_container(content_list).into()
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(200.0)
                        .max_height(1080.0);
                    get_popup(popup_settings)
                }
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::ToggleExampleRow(toggled) => self.example_row = toggled,
            Message::UpdateOutput(output) => self.command_output = output,
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}
