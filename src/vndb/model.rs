use std::collections::HashMap;

use qmetaobject::{prelude::*, USER_ROLE};

#[derive(QObject, Default)]
pub struct VisualNovelListModel {
    base: qt_base_class!(trait QAbstractListModel),
}

impl QAbstractListModel for VisualNovelListModel {
    fn row_count(&self) -> i32 {
        500
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        if role == USER_ROLE {
            QString::from("Visual Novel").into()
        } else if role == USER_ROLE + 1 {
            QString::from("qrc:///assets/placeholderimage.png").into()
        } else {
            QString::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam id risus id augue euismod accumsan. Nunc vestibulum placerat bibendum.").into()
        }
    }

    fn role_names(&self) -> std::collections::HashMap<i32, QByteArray> {
        let mut roles = HashMap::new();
        roles.insert(USER_ROLE, "name".into());
        roles.insert(USER_ROLE + 1, "image".into());
        roles.insert(USER_ROLE + 2, "description".into());
        roles
    }
}
