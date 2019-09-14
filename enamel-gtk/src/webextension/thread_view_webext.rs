use std::{mem, thread};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use log::*;
use env_logger;
use serde_derive::{Serialize, Deserialize};
use glib::Cast;
use glib::Object;
use glib::closure::Closure;
use glib::variant::Variant;
use gio;
use gio::prelude::*;
use gio::{SocketClientExt, IOStreamExt, InputStreamExtManual, OutputStreamExtManual};
use gtk::IconThemeExt;
use webkit2gtk_webextension::{
    DOMDocument,
    DOMDocumentExt,
    DOMElementExt,
    DOMEventTargetExt,
    DOMMouseEvent,
    DOMMouseEventExt,
    WebExtension,
    WebExtensionExt,
    WebPage,
    WebPageExt,
    web_extension_init_with_data
};

use std::os::unix::net::UnixStream;
use async_std::os::unix::net::{UnixStream as AsyncUnixStream};
use async_std::os::unix::io::{AsRawFd, FromRawFd};
use futures::future::{self, Future, FutureExt};

use capnp::Error;
use capnp::primitive_list;
use capnp::capability::Promise;

use capnp_rpc::{RpcSystem, rpc_twoparty_capnp};
use capnp_rpc::twoparty::VatNetwork;

use crate::webext_capnp::page;

web_extension_init_with_data!();


/// Init Gtk and logger.
fn init() {
    use std::sync::{Once, ONCE_INIT};

    static START: Once = ONCE_INIT;

    START.call_once(|| {
        env_logger::init();

        // we're being called in an environment that has gtk already
        // initialized, but gtk-rs does not know that.
        // TODO: move this into webkit2gtk-webextension
        unsafe {
            gtk::set_initialized();
        }
    });
}

const ATTACHMENT_ICON_WIDTH: i32 = 35;


#[derive(Debug, Clone)]
pub struct ThreadViewWebExt{
    extension: WebExtension,
    page: Option<WebPage>
}

impl ThreadViewWebExt{

    pub fn new(extension: webkit2gtk_webextension::WebExtension) -> Self{
        ThreadViewWebExt{
            extension,
            page: None
        }
    }


    pub fn on_page_created(&mut self, page: &webkit2gtk_webextension::WebPage){
        
        info!("WEEEEEE");
        self.page = Some(page.clone());

        page.connect_document_loaded(|page| {
            println!("Page {} created for {:?}", page.get_id(), page.get_uri());
            let document = page.get_dom_document().unwrap();
            println!("URL: {:?}", document.get_url());
            println!("Title: {:?}", document.get_title());
            document.set_title("My Web Page");

            let handler = Closure::new(|values| {
                if let Ok(Some(event)) = values[1].get::<Object>() {
                    // if let Ok(mouse_event) = event.downcast::<DOMMouseEvent>() {
                    //     println!("Click at ({}, {})", mouse_event.get_x(), mouse_event.get_y());
                    // }
                }
                None
            });
            document.add_event_listener_with_closure("click", &handler, false);

            println!("{}%", scroll_percentage(page));
            scroll_by(page, 45);

            println!("{}%", scroll_percentage(page));
            scroll_bottom(page);

            println!("{}%", scroll_percentage(page));
            scroll_top(page);

            println!("{}%", scroll_percentage(page));
        });
    }
}


pub fn web_extension_initialize(extension: &WebExtension, user_data: Option<&Variant>) {
    init();


    /* load attachment icon */
    let theme = gtk::IconTheme::get_default().unwrap();
    let attachment_icon = theme.load_icon(
        "mail-attachment-symbolic",
        ATTACHMENT_ICON_WIDTH,
        gtk::IconLookupFlags::USE_BUILTIN);

    /* load marked icon */
    let marked_icon = theme.load_icon (
        "object-select-symbolic",
        ATTACHMENT_ICON_WIDTH,
        gtk::IconLookupFlags::USE_BUILTIN);


    let user_string: Option<String> = user_data.and_then(Variant::get_str).map(ToOwned::to_owned);
    debug!("user string: {:?}", user_string);

    let socket_addr = user_string.unwrap();


    let mut rstream_sync = UnixStream::connect(socket_addr).unwrap();
    let mut wstream_sync = rstream_sync.try_clone().unwrap();

    let rstream: AsyncUnixStream = rstream_sync.into();
    let wstream: AsyncUnixStream = wstream_sync.into();

    let webext = ThreadViewWebExt{
        extension: extension.clone(),
        page: None
    };
    let page_srv = page::ToClient::new(webext.clone()).into_client::<::capnp_rpc::Server>();

    let network = VatNetwork::new(rstream,
                                  wstream,
                                  rpc_twoparty_capnp::Side::Server,
                                  Default::default());

    let rpc_system = RpcSystem::new(Box::new(network), Some(page_srv.clone().client));

    extension.connect_page_created(move |_, page| {
        let mut cwebext = webext.clone();
        cwebext.on_page_created(page);
    });

    let ctx = glib::MainContext::default();

    ctx.push_thread_default();
    ctx.spawn_local(rpc_system.then(move |_result| {
        // TODO: do something with this result...

        future::ready(())
    }));
    ctx.pop_thread_default();
}

impl page::Server for ThreadViewWebExt
{
    fn allow_remote_images(&mut self,
            params: page::AllowRemoteImagesParams,
            mut results: page::AllowRemoteImagesResults)
            -> Promise<(), Error>
    {
        Promise::ok(())
    }

    fn load(&mut self,
            params: page::LoadParams,
            mut results: page::LoadResults)
            -> Promise<(), Error>
    {

    // load @1(html: Text,
    //         css: Text,
    //         partCss: Text,
    //         allowedUris: List(Text),
    //         useStdout: Bool,
    //         useSyslog: Bool,
    //         disableLog: Bool,
    //         logLevel: Text) -> ();
        // self.extension.

//   GError *err = NULL;
        // let page = self.page.as_ref().unwrap();
        // let document: DOMDocument = page.get_dom_document().unwrap();

        // load html
        info!("loading html..");

        // let he = document.create_element("HTML");
        // let page2 = self.page.as_ref().unwrap();

//   WebKitDOMElement * he = webkit_dom_document_create_element (d, "HTML", (err = NULL, &err));
//   webkit_dom_element_set_outer_html (he, s.html ().c_str (), (err = NULL, &err));

//   webkit_dom_document_set_body (d, WEBKIT_DOM_HTML_ELEMENT(he), (err = NULL, &err));

//   /* load css style */
//   LOG (debug) << "loading stylesheet..";
//   WebKitDOMElement  *e = webkit_dom_document_create_element (d, "STYLE", (err = NULL, &err));

//   WebKitDOMText *t = webkit_dom_document_create_text_node
//     (d, s.css().c_str());

//   webkit_dom_node_append_child (WEBKIT_DOM_NODE(e), WEBKIT_DOM_NODE(t), (err = NULL, &err));

//   WebKitDOMHTMLHeadElement * head = webkit_dom_document_get_head (d);
//   webkit_dom_node_append_child (WEBKIT_DOM_NODE(head), WEBKIT_DOM_NODE(e), (err = NULL, &err));
//   LOG (debug) << "done";

//   /* store part / iframe css for later */
//   part_css = s.part_css ();

//   /* store allowed uris */
//   for (auto &s : s.allowed_uris ()) {
//     allowed_uris.push_back (s);
//   }

//   page_ready = true;

//   g_object_unref (he);
//   g_object_unref (head);
//   g_object_unref (t);
//   g_object_unref (e);
//   g_object_unref (d);

//   ack (true);

//    }

        Promise::ok(())
    }

}



fn scroll_by(page: &WebPage, pixels: i64) {
    let document = page.get_dom_document().unwrap();
    let body = document.get_body().unwrap();
    body.set_scroll_top(body.get_scroll_top() + pixels);
}

fn scroll_bottom(page: &WebPage) {
    let document = page.get_dom_document().unwrap();
    let body = document.get_body().unwrap();
    body.set_scroll_top(body.get_scroll_height());
}

fn scroll_percentage(page: &WebPage) -> i64 {
    let document = page.get_dom_document().unwrap();
    let body = document.get_body().unwrap();
    let document = document.get_document_element().unwrap();
    let height = document.get_client_height();
    (body.get_scroll_top() as f64 / (body.get_scroll_height() as f64 - height) * 100.0) as i64
}

fn scroll_top(page: &WebPage) {
    let document = page.get_dom_document().unwrap();
    let body = document.get_body().unwrap();
    body.set_scroll_top(0);
}


