use std::cmp;
use glib;
use gio;
use gio::{InputStreamExt,
          DataInputStreamExt,
          OutputStreamExt,
          DataOutputStreamExt};
use serde_derive::{Serialize, Deserialize};
use bincode::{serialize, deserialize};
// use futures_core::Future;
use std::boxed::Box as Box_;

const MAX_MESSAGE_SZ: u64 = 200 * 1024 * 1024; // 200 MB


pub enum Error{
    ioError(gio::Error),
    serdeError(bincode::Error),
}

impl From<gio::Error> for Error {
    fn from(err: gio::Error) -> Error {
        Error::ioError(err)
    }
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Error {
        Error::serdeError(err)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Focus {
    message_id: String,
    focus: bool,
    element: i32,
}

#[derive(Deserialize, Serialize)]
pub enum NavigateDirection {
    Undefined,
    Specific,
    Up,
    Down,
}

#[derive(Deserialize, Serialize)]
pub enum NavigateType {
    VisualElement, // Move one element, scrolling if necessary (default movement)
    Visual, // Move one visual step regardless of element, update focus if necessary
    VisualBig, // Move one visual big step, update focus if necessary
    VisualPage, // Move one visual big big step, update focus if necessary
    Element, // Move to specific element, update focus (not directional)
    Message, // Move one message, update focus
    FocusView, // Update focus to match view
    Extreme, // Top or bottom
}


#[derive(Deserialize, Serialize)]
pub enum Message{
    Ack(/*id:*/i32, /*success:*/bool, /*focus:*/Focus),
    Indent(bool),
    AllowRemoteImages(bool),
    Navigate(/*direction:*/NavigateDirection,
             /*type:*/NavigateType,
             /*message_id:*/String,
             /*element:*/i32,
             /*focus_top:*/bool),

    Mark(/*message_id:*/String,
         /*marked:*/bool),

    Hidden(/*message_id:*/String,
           /*hidden:*/bool),

    Debug(/*msg:*/String)
}




pub trait MessageInputStream{
    fn read_message<'a, P: glib::IsA<gio::Cancellable> + 'a, Q: Into<Option<&'a P>>>(self, cancellable: Q) -> Result<Message, Error>;

}

impl MessageInputStream for gio::DataInputStream{
    fn read_message<'a, P: glib::IsA<gio::Cancellable> + 'a, Q: Into<Option<&'a P>>>(self, cancellable: Q) -> Result<Message, Error>
    {
        let cancl = cancellable.into();
        let msg_len = self.read_uint64(cancl)?;
        let msg_size = cmp::min(msg_len, MAX_MESSAGE_SZ);
        let msg_b = self.read_bytes(msg_size as usize, cancl)?;

        match deserialize::<Message>(&msg_b){
            Ok(msg) => Ok(msg),
            Err(err) => Err(Error::serdeError(err))
        }
    }
}

pub trait MessageOutputStream{
    fn write_message<'a, P: glib::IsA<gio::Cancellable> + 'a, Q: Into<Option<&'a P>>>(self, msg: Message, cancellable: Q) -> Result<(), Error>;

}

impl MessageOutputStream for gio::DataOutputStream{
    fn write_message<'a, P: glib::IsA<gio::Cancellable> + 'a, Q: Into<Option<&'a P>>>(self, msg: Message, cancellable: Q) -> Result<(), Error>
    {
        let cancl = cancellable.into();
        let msg_ser = serialize(&msg)?;
        let msg_len = msg_ser.len();
        self.put_uint64(msg_len as u64, cancl)?;
        let msg_b = glib::Bytes::from(&msg_ser);
        self.write_bytes(&msg_b, cancl)?;

        Ok(())
    }
}



// fn send_message_future(os: gio::OutputStream, msg:Message) -> Box_<Future<Item = (gio::OutputStream, isize), Error = (gio::OutputStream, Error)>> where Self: Sized + Clone;
//  {
//     let encoded = serialize(&msg).unwrap();
//     let sz = encoded.len();

//     os.wr


// }


// message Indent {
//   string bogus = 1;
//   bool   indent = 2;
// }

// message AllowRemoteImages {
//   string bogus = 1;
//   bool   allow = 2;
// }

// message Navigate {
//   enum Direction {
//     None     = 0;
//     Specific = 1;
//     Up       = 2;
//     Down     = 3;
//   }

//   enum Type {
//     VisualElement = 0; // Move one element, scrolling if necessary (default movement)
//     Visual        = 1; // Move one visual step regardless of element, update focus if necessary
//     VisualBig     = 2; // Move one visual big step, update focus if necessary
//     VisualPage    = 3; // Move one visual big big step, update focus if necessary
//     Element       = 4; // Move to specific element, update focus (not directional)
//     Message       = 5; // Move one message, update focus
//     FocusView     = 6; // Update focus to match view
//     Extreme       = 7; // Top or bottom
//   }

//   Direction direction = 1;
//   Type      type = 2;

//   /* for Element */
//   string mid = 3;
//   int32  element = 4;

//   /* for message */
//   bool focus_top = 5;
// }

// message Mark {
//   string mid = 1;
//   bool   marked = 2;
// }

// message Hidden {
//   string mid = 1;
//   bool   hidden = 2;
// }

// message Debug {
//   string msg = 1;
// }

// message Page {
//   string html = 1;
//   string css = 2;
//   string part_css = 3;

//   repeated string allowed_uris = 4;

//   bool   use_stdout = 5;
//   bool   use_syslog = 6;
//   bool   disable_log = 7;
//   string log_level = 8;
// }

// message Info {
//   bool warning = 1;
//   bool set = 2;
//   string mid = 3;
//   string txt = 4;
// }

// message Address {
//   string name = 1;
//   string email = 2;
//   string full_address = 3;
// }

// message AddressList {
//   repeated Address addresses = 1;
// };

// message Message {
//   string mid = 1;

//   Address     sender = 2;
//   AddressList to = 3;
//   AddressList cc = 4;
//   AddressList bcc = 5;
//   Address     reply_to = 20;

//   string date_pretty = 6;
//   string date_verbose = 7;

//   string subject = 9;

//   repeated string tags = 10;
//   string          tag_string = 21;

//   string gravatar = 11;
//   bool   missing_content = 13;
//   bool   patch = 14;
//   bool   different_subject = 22;
//   int32  level = 15;
//   string in_reply_to = 16;

//   string preview = 17;


//   message Chunk {
//     int32 id = 1;
//     string sid = 13;
//     string mime_type = 6;
//     string cid = 22;
//     bool viewable = 2;
//     bool preferred = 3;
//     bool attachment = 7;

//     bool  is_encrypted = 8;
//     bool  is_signed = 9; // 'signed' doesn't work
//     int32 crypto_id = 21;

//     message Signature {
//       bool verified = 1;

//       repeated string sign_strings = 2;
//       repeated string all_errors = 3;
//     }

//     message Encryption {
//       bool decrypted = 1;
//       repeated string enc_strings = 2;
//     }

//     Signature   signature  = 19;
//     Encryption  encryption = 20;


//     bool sibling = 11;
//     bool use = 12;
//     bool focusable = 18;

//     string content = 10;
//     string filename = 14;
//     int32  size = 15;
//     string human_size = 16;

//     string thumbnail = 17; // used by attachments

//     repeated Chunk kids = 4;
//     repeated Chunk siblings = 5;
//   }

//   Chunk root = 23;

//   repeated Chunk mime_messages = 18;
//   repeated Chunk attachments = 19;
// }


// /* This should match the state structures in thread_view.hh */
// message State {
//   message MessageState {
//     string mid = 1;

//     message Element {
//       enum Type {
//         Empty       = 0;
//         Address     = 1;
//         Part        = 2;
//         Attachment  = 3;
//         MimeMessage = 4;
//         Encryption  = 5;
//       }

//       Type   type = 1;
//       int32  id   = 2;
//       string sid  = 3;
//       bool   focusable = 4;
//     }

//     repeated Element elements = 5;
//     int32    level = 6;
//   }

//   repeated MessageState messages = 2;
//   bool edit_mode = 3;
// }

// message UpdateMessage {
//   Message m = 1;

//   enum Type {
//     Tags         = 0;
//     VisibleParts = 1;
//   }

//   Type type = 2;
// }

// message ClearMessage {
//   bool yes = 1;
// }




