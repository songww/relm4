use gtk::prelude::{FileChooserExt, FileExt, NativeDialogExt};
use relm4::{send, ComponentUpdate, Model, Sender};

use std::path::PathBuf;

pub struct SaveDialogSettings {
    pub cancel_label: String,
    pub accept_label: String,
    pub create_folders: bool,
    pub is_modal: bool,
    pub filters: Vec<gtk::FileFilter>,
}

#[tracker::track]
pub struct SaveDialogModel {
    #[do_not_track]
    settings: SaveDialogSettings,
    suggestion: Option<String>,
    is_active: bool,
    name: String,
}

pub enum SaveDialogMsg {
    Save,
    SaveAs(String),
    Accept(PathBuf),
    InvalidInput,
    Cancel,
}

impl Model for SaveDialogModel {
    type Msg = SaveDialogMsg;
    type Widgets = SaveDialogWidgets;
    type Components = ();
}

pub trait SaveDialogParent: Model
where
    Self::Widgets: SaveDialogParentWidgets,
{
    fn dialog_config(&self) -> SaveDialogSettings;
    fn save_msg(path: PathBuf) -> Self::Msg;
}

pub trait SaveDialogParentWidgets {
    fn parent_window(&self) -> Option<gtk::Window>;
}

impl<ParentModel> ComponentUpdate<ParentModel> for SaveDialogModel
where
    ParentModel: SaveDialogParent,
    <ParentModel as relm4::Model>::Widgets: SaveDialogParentWidgets,
{
    fn init_model(parent_model: &ParentModel) -> Self {
        SaveDialogModel {
            settings: parent_model.dialog_config(),
            is_active: false,
            suggestion: None,
            name: String::new(),
            tracker: 0,
        }
    }

    fn update(
        &mut self,
        msg: SaveDialogMsg,
        _components: &(),
        _sender: Sender<SaveDialogMsg>,
        parent_sender: Sender<ParentModel::Msg>,
    ) {
        self.reset();

        match msg {
            SaveDialogMsg::Save => {
                self.is_active = true;
            }
            SaveDialogMsg::SaveAs(name) => {
                self.is_active = true;
                self.set_name(name);
            }
            SaveDialogMsg::Cancel => {
                self.is_active = false;
            }
            SaveDialogMsg::Accept(path) => {
                self.is_active = false;
                parent_sender.send(ParentModel::save_msg(path)).unwrap();
            }
            _ => (),
        }
    }
}

#[relm4_macros::widget(pub)]
impl<ParentModel> relm4::Widgets<SaveDialogModel, ParentModel> for SaveDialogWidgets
where
    ParentModel: Model,
    ParentModel::Widgets: SaveDialogParentWidgets,
{
    //type Model = SaveDialogModel;

    view! {
        gtk::FileChooserNative {
            set_action: gtk::FileChooserAction::Save,
            set_visible: watch!(model.is_active),
            set_current_name: track!(model.changed(SaveDialogModel::name()), &model.name),
            add_filter: iterate!(&model.settings.filters),
            set_create_folders: model.settings.create_folders,
            set_cancel_label: Some(&model.settings.cancel_label),
            set_accept_label: Some(&model.settings.accept_label),
            set_modal: model.settings.is_modal,
            set_transient_for: parent_widgets.parent_window().as_ref(),
            connect_response => move |dialog, res_ty| {
                match res_ty {
                    gtk::ResponseType::Accept => {
                        if let Some(file) = dialog.file() {
                            if let Some(path) = file.path() {
                                send!(sender, SaveDialogMsg::Accept(path));
                                return;
                            }
                        }
                        send!(sender, SaveDialogMsg::InvalidInput);
                    },
                    gtk::ResponseType::Cancel => {
                        send!(sender, SaveDialogMsg::Cancel)
                    },
                    _ => (),
                }
            },
        }
    }
}
