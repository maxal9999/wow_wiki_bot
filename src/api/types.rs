use rustc_serialize::{Encodable, Decodable, Encoder, Decoder};

macro_rules! impl_encode {
    (
        $ty:ident, $count:expr,
        [$($id:expr => $field:ident),*],
        [$($o_id:expr => $o_field:ident),*]
    ) => {
        impl Encodable for $ty {
            fn encode<E: Encoder>(&self, e: &mut E) -> std::result::Result<(), E::Error> {
                e.emit_struct(stringify!($ty), $count, |e| {
                    $(
                        try!(e.emit_struct_field(stringify!($field), $id, |e| {
                            self.$field.encode(e)
                        }));
                    )*
                    $(
                        if let Some(ref v) = self.$o_field {
                            try!(e.emit_struct_field(
                                stringify!($o_field), $o_id, |e| {
                                v.encode(e)
                            }));
                        }
                    )*

                    Ok(())
                })
            }
        }
    }
}

// Decodes a field with a given name
macro_rules! try_field {
    ($d:ident, $name:expr) => {
        try!($d.read_struct_field($name, 0, Decodable::decode))
    }
}

/// Structure for formed full endpoint
pub struct Pair<'a> {
    key: &'a str,
    value: String
}

impl<'a> Clone for Pair<'a> {
    fn clone(&self) -> Pair<'a> {
        Pair {
            key: self.key.clone(),
            value: self.value.clone()
        }
    }
}

/// Structure for formed full endpoint
pub struct UrlParams<'a> {
    map: Vec<Pair<'a>>,
}

impl<'a> UrlParams<'a> {
    pub fn new() -> UrlParams<'a> {
        UrlParams {
            map: Vec::new(),
        }
    }

    pub fn add_opt_value<T: ToString>(&mut self, key: &'a str, opt_value: Option<T>) {
        if let Some(value) = opt_value {
            self.map.push(Pair{ key: key, value: value.to_string() });
        }
    }
    pub fn add_value<T: ToString>(&mut self, key: &'a str, value: T) {
        self.map.push(Pair{ key: key, value: value.to_string() });
    }

    pub fn get_url_string(self) -> String {
        let map_clone = self.map;
        map_clone.into_iter().map(|ref mut tmp_pair| {
            format!("{}={}", tmp_pair.key, &*tmp_pair.value)
        }).collect::<Vec<_>>().join("&")
    }
}

/// Type for URL response.
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct Response<T: Decodable> {
    pub ok: bool,
    pub error_code: Option<i64>,
    pub description: Option<String>,
    pub result: Option<T>,
}

/// Telegram type "Chat"
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct Chat {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>
}

impl_encode!(Chat, 3,
    [0 => id, 1 => first_name],
    [2 => last_name]);

/// Telegram type "User"
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub is_bot: bool,
    pub language_code: Option<String>,
    pub last_name: Option<String>
}

impl_encode!(User, 4,
    [0 => id, 1 => first_name],
    [2 => language_code, 3 => last_name]);

/// Telegram type "Message"
#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub chat: Chat,
    pub date: i64,
    pub from: User,
    pub message_id: i64,
    pub text: Option<String>
}

impl Decodable for Message {
    fn decode<D: Decoder>(decoder: &mut D) -> std::result::Result<Self, D::Error> {
        decoder.read_struct("Message", 0, |d| {
            Ok(Message {
                chat: try_field!(d, "chat"),
                date: try_field!(d, "date"),
                from: try_field!(d, "from"),
                message_id: try_field!(d, "message_id"),
                text: try_field!(d, "text"),
            })
        })
    }
}

/// Telegram type "Update" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>
}