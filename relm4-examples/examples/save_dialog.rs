use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{
    impl_model, send, AppUpdate, Components, RelmApp, RelmComponent, Sender, WidgetPlus, Widgets,
};
use relm4_components::save_dialog::{
    SaveDialogMsg, SaveDialogParent, SaveDialogSettings, SaveDialogWidgets,
};

use std::path::PathBuf;

#[derive(Default)]
pub struct AppModel {
    counter: u8,
}

pub enum AppMsg {
    Increment,
    Decrement,
    SaveRequest,
    SaveResponse(PathBuf),
}

impl_model!(AppModel, AppMsg, AppComponents);

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, components: &AppComponents, _sender: Sender<AppMsg>) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
            AppMsg::SaveRequest => {
                components
                    .dialog
                    .send(SaveDialogMsg::SaveAs(format!("Counter_{}", self.counter)))
                    .unwrap();
            }
            AppMsg::SaveResponse(path) => {
                println!("File would have been saved at {:?}", path);
            }
        }
    }
}

impl SaveDialogParent for AppModel {
    fn dialog_config(&self) -> SaveDialogSettings {
        SaveDialogSettings {
            accept_label: "Open".to_string(),
            cancel_label: "Cancel".to_string(),
            create_folders: true,
            filters: Vec::new(),
        }
    }

    fn save_msg(path: PathBuf) -> Self::Msg {
        AppMsg::SaveResponse(path)
    }
}

pub struct AppComponents {
    dialog: RelmComponent<SaveDialogWidgets, AppModel>,
}

impl Components<AppModel> for AppComponents {
    fn init_components(model: &AppModel, sender: Sender<AppMsg>) -> Self {
        AppComponents {
            dialog: RelmComponent::new(model, sender),
        }
    }
}

#[relm4_macros::widget]
impl Widgets for AppWidgets {
    type Model = AppModel;

    view! {
        gtk::ApplicationWindow {
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 100,
            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                append = &gtk::Button {
                    set_label: "Increment",
                    connect_clicked => move |_| {
                        send!(sender, AppMsg::Increment);
                    },
                },
                append = &gtk::Button {
                    set_label: "Decrement",
                    connect_clicked => move |_| {
                        send!(sender, AppMsg::Decrement);
                    },
                },
                append = &gtk::Label {
                    set_margin_all: 5,
                    set_label: watch! { &format!("Counter: {}", model.counter) },
                },
                append = &gtk::Button {
                    set_label: "Save",
                    connect_clicked => move |_| {
                        send!(sender, AppMsg::SaveRequest);
                    },
                },
            },
        }
    }
}

fn main() {
    let model = AppModel::default();
    let app: RelmApp<AppWidgets> = RelmApp::new(model);
    app.run();
}