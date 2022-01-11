mod vndb;

use cstr::cstr;
use ki18n::klocalizedcontext::KLocalizedContext;
use qmetaobject::prelude::*;
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
        "placeholderimage/256px-placeholder.png" as "placeholderimage.png"
    }
);

fn main() {
    qmetaobject::log::init_qt_to_rust();
    env_logger::init();

    qml_register_type::<vndb::model::VisualNovelListModel>(
        cstr!("VisualNovelListModel"),
        1,
        0,
        cstr!("VisualNovelListModel"),
    );

    let mut engine = QmlEngine::new();

    KLocalizedContext::init_from_engine(&engine);

    root_qml();
    vndb_qml();
    assets();

    engine.load_url(QUrl::from(QString::from("qrc:///main.qml")));

    engine.exec();
}
