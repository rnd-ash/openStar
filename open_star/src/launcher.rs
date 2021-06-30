use hardware::HardwareAPI;
use iced::{Align, Color, Column, Row, Text};
use logger::Logger;
use nfd2::Response;
use crate::widgets::{column, create_button, main_text, radio_btn, row};

use super::widgets;

#[derive(Debug, Clone)]
pub enum LauncherMsg {
    SelectFolderPress,
    LaunchPress,
    
    SelectAPI(HardwareAPI),
    HardwareSelected(String),
}

#[derive(Debug, Clone, Default)]
pub struct Launcher {
    path_valid: bool,
    path_select_state: iced::button::State,
    launch_btn_state: iced::button::State,
    root_path: String,
    launch_ready: bool,
    device_valid: bool,
    device_dropdown_state: iced::pick_list::State<String>,
    exit: bool,
    logger: Logger,
    api: HardwareAPI,
    selected_hw: Option<String>,
    device_list: Vec<String>,
    error: Option<String>
}

impl iced::Application for Launcher {
    type Executor = iced::executor::Default;

    type Message = LauncherMsg;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                path_valid: false,
                root_path: "".into(),
                launch_ready: false,
                logger: Logger::new("OpenStar Launcher"),
                ..Default::default()
            },
            iced::Command::none()
        )
    }

    fn title(&self) -> String {
        "OpenStar Launcher".into()
    }

    fn should_exit(&self) -> bool {
        self.exit
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match message {
            LauncherMsg::SelectFolderPress => {
                #[cfg(Windows)]
                let default_path = Some(r"C:\Program Files (x86)\");
                #[cfg(unix)]
                let default_path = None;
                self.path_valid = false;
                self.launch_ready = false;
                self.root_path.clear();
                match nfd2::open_pick_folder(default_path).unwrap() {
                    Response::Okay(p) => {
                        let das_path = p.clone().join("DAS");
                        let xen_path = p.clone().join("Xentry"); 
                    
                        if das_path.exists() {
                            self.logger.log_debug(format!("DAS install located at {:?}", das_path));
                            filehandler::set_das_path(das_path);
                        } else {
                            self.logger.log_err("Path contains no DAS install!".into());
                            self.error = Some(format!("{:?} does not contain a DAS installation", p));
                            return iced::Command::none()
                        }
                        if xen_path.exists() {
                            self.logger.log_debug(format!("Xentry install located at {:?}", xen_path));
                            filehandler::set_xentry_path(xen_path);
                        } else {
                            self.logger.log_err("Path contains no Xentry install!".into());
                            self.error = Some(format!("{:?} does not contain a Xentry installation", p));
                            return iced::Command::none()
                        }
                        // path is OK
                        self.error = None;
                        self.path_valid = true;
                        self.root_path = format!("{:?}", p);
                        if self.selected_hw.is_some() {
                            self.launch_ready = true;
                        }
                    },
                    _ => {
                        self.error = Some("No path was selected".into());
                    }
                }
            },
            LauncherMsg::LaunchPress => {
                if self.launch_ready {
                    if hardware::open_device(&self.selected_hw.clone().unwrap(), self.api) {
                        self.exit = true;
                        unsafe { super::launcher_ok = true };
                    } else {

                    }
                }
            },

            LauncherMsg::SelectAPI(api) => {
                if api == self.api {
                    return iced::Command::none()
                }
                self.selected_hw = None;
                self.logger.log_debug(format!("Device API '{:?}' selected", api));
                self.device_list = hardware::get_device_list(api);
                self.api = api;
            },
            LauncherMsg::HardwareSelected(dev) => {
                self.selected_hw = Some(dev);
                if self.path_valid {
                    self.launch_ready = true;
                }
            }
        }
        iced::Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {

        let p1 = main_text("To begin, please select your Mercedes-Benz folder. This will be your root DAS and Xentry install folder");

        let mut master_layout = column().height(iced::Length::Fill);

        master_layout = master_layout.push(p1);

        master_layout = master_layout.push(iced::rule::Rule::horizontal(2));
        let mut row1 = row().push(
            create_button(&mut self.path_select_state, "Select Mercedes-Benz folder").on_press(LauncherMsg::SelectFolderPress)
        ).push(iced::Space::with_width(iced::Length::Fill));

        if self.path_valid {
            row1 = row1.push(main_text(format!("{} is valid", self.root_path)).color(*widgets::GREEN))
        } else if !self.path_valid && self.error.is_some() {
            row1 = row1.push(main_text(self.error.clone().unwrap()).color(*widgets::RED))
        } else {
            row1 = row1.push(main_text("No path selected"));
        }

        master_layout = master_layout.push(row1);

        master_layout = master_layout.push(iced::rule::Rule::horizontal(2));


        let mut launch_btn = create_button(&mut self.launch_btn_state, "Launch OpenStar");
        let mut text: String = "Not ready to launch".into();
        if self.launch_ready {
            text = "Ready to launch!".into();
            launch_btn = launch_btn.on_press(LauncherMsg::LaunchPress)
        }


        if self.path_valid {
            let mut row2 = column().align_items(Align::Start);

            row2 = row2.push(main_text("Physical devices:"));
            row2 = row2.push(radio_btn(HardwareAPI::Passthru, "Passthru", Some(self.api),LauncherMsg::SelectAPI));
            row2 = row2.push(radio_btn(HardwareAPI::Sd, "SDConnect", Some(self.api),LauncherMsg::SelectAPI));
            row2 = row2.push(radio_btn(HardwareAPI::Pdu, "D-PDU", Some(self.api),LauncherMsg::SelectAPI));
            #[cfg(unix)]
            {
                row2 = row2.push(radio_btn(HardwareAPI::SocketCAN, "SocketCAN", Some(self.api),LauncherMsg::SelectAPI));
            }
            row2 = row2.push(main_text("Special devices:"));
            row2 = row2.push(radio_btn(HardwareAPI::Sim, "SIMULATION", Some(self.api),LauncherMsg::SelectAPI));


            master_layout = master_layout.push(row2);

            let mut row3 = row();
            let text = if self.device_list.is_empty() {
                main_text(format!("No {} devices located", self.api)).color(*widgets::RED)
            } else if self.device_list.len() == 1{
                main_text(format!("'{}' located", self.device_list[0]))
            } else {
                main_text(format!("Multiple {} devices located. Please choose", self.api))
            };

            row3 = row3.push(text).align_items(Align::Center);

            master_layout = master_layout.push(row3);

            if self.device_list.len() > 1 {
                master_layout = master_layout.push(
                    iced::pick_list::PickList::new(&mut self.device_dropdown_state, &self.device_list, self.selected_hw.clone(), LauncherMsg::HardwareSelected)
                )   
            }

        }

        let row_bottom = row()
        .push(iced::Space::with_width(iced::Length::Fill))
        .push(main_text(&text))
        .push(launch_btn);

        master_layout = master_layout.push(iced::Space::with_height(iced::Length::Fill));
        master_layout = master_layout.push(row_bottom);

        iced::Container::new(master_layout).height(iced::Length::Fill).width(iced::Length::Fill).padding(5).into()
    }
}