use std::rc::Rc;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;

use notmuch;
use notmuch::DatabaseMode;

use enamel_core::settings::Settings;
use enamel_core::database::Manager as DBManager;


// pub struct TagList {
//     pub container: gtk::TreeView,
//
//     model: gtk::ListStore,
//
//
//     dbmanager: Arc<DBManager>
// }
//
//

fn append_text_column(tree: &gtk::TreeView, id: i32) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}


// impl TagList {
//     pub fn new(dbmanager: Arc<DBManager>) -> Self {
//
//         let model = gtk::ListStore::new(&[String::static_type()]);
//
//
//         let container = gtk::TreeView::new_with_model(&model);
//         container.set_headers_visible(false);
//         append_text_column(&container, 0);
//
//
//         let mut tl = TagList {
//             container,
//             model,
//             dbmanager,
//         };
//
//         return tl;
//     }
//
//     pub fn refresh(self: &mut Self){
//
//         self.model.clear();
//
//         let mut dbman = self.dbmanager.clone();
//
//         let db = dbman.get(DatabaseMode::ReadOnly).unwrap();
//
//         let mut tags = db.all_tags().unwrap();
//
//         loop {
//             match tags.next() {
//                 Some(tag) => {
//                     self.add_tag(&tag);
//                 },
//                 None => { break }
//             }
//         }
//
//
//     }
//

//
// }

// #[derive(Msg)]
// pub enum Msg {
//     Refresh,
//     SelectionChanged,
//     ItemSelect(Option<String>)
// }

pub struct TagList {
    model: TagListModel,
    scrolled_window: gtk::ScrolledWindow,
    tree_view: gtk::TreeView,
    tree_model: gtk::ListStore
}

pub struct TagListModel {
    // relm: ::relm::Relm<TagList>,
    settings: Rc<Settings>,
    dbmanager: Arc<DBManager>,
}

impl TagList{
    fn refresh(&mut self){
        let mut dbman = self.model.dbmanager.clone();
        let db = dbman.get(DatabaseMode::ReadOnly).unwrap();
        let mut tags = db.all_tags().unwrap();

        loop {
         match tags.next() {
             Some(tag) => {
                 self.add_tag(&tag);
             },
             None => { break }
         }
        }
    }


    fn add_tag(self: &mut Self, tag: &String){
        let it = self.tree_model.append();
        self.tree_model.set_value(&it, 0, &tag.to_value());
    }

    fn on_selection_changed(self: &mut Self){
        let (model, iter) = self.tree_view.get_selection().get_selected().unwrap();


        // if(self.tree_model.iter_is_valid(&iter)){
        //     let val: String = model.get_value(&iter, 0).get().unwrap();
        //     self.model.relm.stream().emit(Msg::ItemSelect(Some(val)));
        // }else{
        //     self.model.relm.stream().emit(Msg::ItemSelect(None));
        // }
    }
}


// impl ::relm::Update for TagList {
//     type Model = TagListModel;
//     type ModelParam = (Rc<Settings>, Arc<DBManager>);
//     type Msg = Msg;

//     fn model(relm: &::relm::Relm<Self>, (settings, dbmanager): Self::ModelParam) -> Self::Model {
//         TagListModel {
//             relm: relm.clone(),
//             settings,
//             dbmanager
//         }
//     }

//     fn update(&mut self, event: Self::Msg) {
//         match event {
//             Msg::Refresh => self.refresh(),
//             Msg::SelectionChanged => self.on_selection_changed(),
//             Msg::ItemSelect(_) => ()
//         }
//     }
// }


impl ::relm::Widget for TagList {

    type Root = gtk::ScrolledWindow;

    fn root(&self) -> Self::Root {
        self.scrolled_window.clone()
    }

    fn view(rlm: &Relm<Self>, model: Self::Model) -> Self
    {
        let scrolled_window = gtk::ScrolledWindow::new(None, None);

        let tree_model = gtk::ListStore::new(&[String::static_type()]);
        let tree_view = gtk::TreeView::new_with_model(&tree_model);
        tree_view.set_headers_visible(false);
        append_text_column(&tree_view, 0);

        scrolled_window.add(&tree_view);

        connect!(relm, tree_view.get_selection(), connect_changed(_), Msg::SelectionChanged);

        TagList {
            model,
            scrolled_window,
            tree_view,
            tree_model,
        }
    }
}
