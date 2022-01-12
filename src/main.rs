mod trackers;

use cstr::cstr;
use ki18n::klocalizedcontext::KLocalizedContext;
use qmetaobject::prelude::*;
use qmetaobject::qtdeclarative::qml_register_singleton_type;
use qmetaobject::QUrl;

qrc!(root_qml,
    "qml" as "" {
        "main.qml",
    }
);

qrc!(vndb_qml,
    "qml/vndb" as "vndb" {
        "vndb.qml",
        "login.qml",
    }
);

qrc!(assets,
    "assets" as "assets" {
        "placeholderimage/256px-placeholder.png" as "placeholderimage.png",
        "banner.jpg" as "banner.jpg"
    }
);

fn main() {
    qmetaobject::log::init_qt_to_rust();
    env_logger::init();

    // test();

    qml_register_type::<trackers::vndb::model::VisualNovelListModel>(
        cstr!("VisualNovelListModel"),
        1,
        0,
        cstr!("VisualNovelListModel"),
    );

    qml_register_singleton_type::<trackers::vndb::data::VNDB>(cstr!("VNDB"), 1, 0, cstr!("VNDB"));

    let mut engine = QmlEngine::new();

    KLocalizedContext::init_from_engine(&engine);

    root_qml();
    vndb_qml();
    assets();

    engine.load_url(QUrl::from(QString::from("qrc:///main.qml")));

    engine.exec();
}

fn test() {
    log::info!("Init Called");

    match vndb::client::Simple::connect_tls() {
        Ok(client) => {
            log::info!("Connection Success");
        }
        Err(e) => log::error!("{}", e),
    }
}
