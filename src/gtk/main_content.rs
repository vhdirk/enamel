use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use gio;
use glib;
use glib::translate::FromGlib;
use gtk;
use gtk::prelude::*;

use notmuch;

use inox_core::settings::Settings;
use inox_core::database::Manager as DBManager;

use tag_list::TagList;

pub struct MainContent {
    pub container: gtk::Paned,
    pub tag_list: TagList

    // pub source:    Source,
    // pub preview:   WebView,
}


impl MainContent {
    pub fn new(dbmanager: Rc<DBManager>) -> Self {
        // Create the Paned container for the main content
        let container = gtk::Paned::new(gtk::Orientation::Horizontal);
        let mut tag_list = TagList::new(dbmanager.clone());

        tag_list.refresh();
        // Create a the WebView for the preview pane.
        // let context = WebContext::get_default().unwrap();
        // let preview = WebView::new_with_context(&context);
        //
        // Pack it in
        container.pack1(&tag_list.container, true, true);
        // container.pack2(&preview, true, true);
        //
        // // Ensure that the two panes get half the size of the paned container.
        // source.container.set_size_request(100, -1);
        // preview.set_size_request(100, -1);

        MainContent { container, tag_list }
    }
}
