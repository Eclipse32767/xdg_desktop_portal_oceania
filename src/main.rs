use std::collections::HashMap;
use zbus::{Connection, dbus_interface, zvariant};
use serde::Serialize;

const PORTAL_RESPONSE_SUCCESS: u32 = 0;
const PORTAL_RESPONSE_CANCELLED: u32 = 1;
const PORTAL_RESPONSE_OTHER: u32 = 2;
#[derive(zvariant::Type)]
#[zvariant(signature = "(ua{sv})")]
enum PortalResponse<T: zvariant::Type + serde::Serialize> {
    Success(T),
    Cancelled,
    Other,
}

impl<T: zvariant::Type + serde::Serialize> serde::Serialize for PortalResponse<T> {
    //noinspection RsMainFunctionNotFound
    //noinspection RsMainFunctionNotFound
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Success(res) => (PORTAL_RESPONSE_SUCCESS, res).serialize(serializer),
            Self::Cancelled => (
                PORTAL_RESPONSE_CANCELLED,
                HashMap::<String, zvariant::Value>::new(),
            )
                .serialize(serializer),
            Self::Other => (
                PORTAL_RESPONSE_OTHER,
                HashMap::<String, zvariant::Value>::new(),
            )
                .serialize(serializer),
        }
    }
}

struct Greeter {
    count: i32
}

#[dbus_interface(name = "org.me.mine")]
impl Greeter {
    async fn say_hello(&mut self, name: &str) -> String {
        self.count = self.count + 1;
        format!("Hello {}! The Count is: {}", name, self.count)
    }
}

struct Request;

#[dbus_interface(name = "org.freedesktop.impl.portal.Request")]
impl Request {
    fn close(&self) {

    }
}
struct Session {
    close_cb: Option<Box<dyn FnOnce() + Send + Sync + 'static>>,
}

impl Session {
    fn new<F: FnOnce() + Send + Sync + 'static>(cb: F) -> Self {
        Self {
            close_cb: Some(Box::new(cb)),
        }
    }
}

#[dbus_interface(name = "org.freedesktop.impl.portal.Session")]
impl Session {
    async fn close(&mut self, #[zbus(signal_context)] signal_context: zbus::SignalContext<'_>) {
        // XXX error?
        let _ = self.closed(&signal_context).await;
        let _ = signal_context
            .connection()
            .object_server()
            .remove::<Self, _>(signal_context.path())
            .await;
        if let Some(cb) = self.close_cb.take() {
            cb();
        }
    }

    #[dbus_interface(signal)]
    async fn closed(&self, signal_context: &zbus::SignalContext<'_>) -> zbus::Result<()>;

    #[dbus_interface(property, name = "version")]
    fn version(&self) -> u32 {
        1 // XXX?
    }
}
//noinspection RsUnresolvedReference
// Although we use `async-std` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let greeter = Greeter { count: 0 };
    // setup the server
    connection
        .object_server()
        .at("/org/zbus/MyGreeter", greeter)
        .await?;
    // before requesting the name
    connection
        .request_name("org.zbus.MyGreeter")
        .await?;

    loop {
        // do something else, wait forever or timeout here:
        // handling D-Bus messages is done in the background
        std::future::pending::<()>().await;
    }
}