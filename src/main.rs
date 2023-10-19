use zbus::{Connection, dbus_interface, Result};


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

#[zbus::dbus_interface(name = "org.freedesktop.impl.portal.Session")]
impl Session {
    async fn close(&mut self, #[zbus(signal_context)] signal_ctxt: zbus::SignalContext<'_>) {
        // XXX error?
        let _ = self.closed(&signal_ctxt).await;
        let _ = signal_ctxt
            .connection()
            .object_server()
            .remove::<Self, _>(signal_ctxt.path())
            .await;
        if let Some(cb) = self.close_cb.take() {
            cb();
        }
    }

    #[dbus_interface(signal)]
    async fn closed(&self, signal_ctxt: &zbus::SignalContext<'_>) -> zbus::Result<()>;

    #[dbus_interface(property, name = "version")]
    fn version(&self) -> u32 {
        1 // XXX?
    }
}
// Although we use `async-std` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<()> {
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