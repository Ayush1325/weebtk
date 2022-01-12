use std::net::TcpStream;

use qmetaobject::{prelude::*, qtdeclarative::QSingletonInit};

use vndb::{client::Simple, protocol::message::request::Login, protocol::Request};

#[derive(QObject, Default)]
pub struct VNDB {
    base: qt_base_class!(trait QObject),
    login: qt_method!(
        fn login(&mut self, username: String, password: String) {
            log::info!("Username: {}", username);
            log::info!("Password: {}", password);
            //            if let Some(c) = &mut self.client {
            //                let login_request = Login::new(Some((username.as_str(), password.as_str())));
            //                if let Err(e) = c.send(&Request::Login(login_request)) {
            //                    log::error!("{}", e);
            //                } else {
            //                    log::info!("Login Success");
            //                }
            //            } else {
            //                log::error!("Error connecting to VNDB");
            //            }
        }
    ),
    client: Option<Simple<TcpStream>>,
}

impl QSingletonInit for VNDB {
    fn init(&mut self) {
        log::info!("Init Called");

        match Simple::connect() {
            Ok(client) => {
                self.client = Some(client);
                log::info!("Connection Success");
            }
            Err(e) => log::error!("{}", e),
        }
    }
}
