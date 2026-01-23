use std::collections::HashMap;
use std::error::Error;

use zbus::{Connection, proxy, zvariant::Value};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let connection = Connection::session().await?;
//
//     let m = connection
//         .call_method(
//             Some("org.freedesktop.Notifications"),
//             "/org/freedesktop/Notifications",
//             Some("org.freedesktop.Notifications"),
//             "Notify",
//             &(
//                 "my-app",
//                 0u32,
//                 "dialog-information",
//                 "A summary",
//                 "Some body",
//                 // they both mean to create a vector (empty vector) of type &str
//                 vec![""; 0],
//                 // Vec::<&str>::new(),
//                 HashMap::<&str, &Value>::new(),
//                 5000,
//             ),
//         )
//         .await?;
//     let reply: u32 = m.body().deserialize().unwrap();
//     dbg!(reply);
//     Ok(())
// }

#[proxy(
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_i32: u32,
        app_icons: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::session().await?;
    let proxy = NotificationsProxy::new(&connection).await?;
    let reply = proxy
        .notify(
            "my_app",
            0,
            "dialog-information",
            "A summary",
            "Some body",
            &[],
            HashMap::new(),
            5000,
        )
        .await?;
    dbg!(reply);

    Ok(())
}
