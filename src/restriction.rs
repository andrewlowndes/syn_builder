use crate::IntoPath;
use syn::{token, FieldMutability, VisRestricted, Visibility};

pub trait IntoVisibility {
    fn into_visibility(self) -> Visibility;
}

impl IntoVisibility for Visibility {
    fn into_visibility(self) -> Visibility {
        self
    }
}

impl IntoVisibility for token::Pub {
    fn into_visibility(self) -> Visibility {
        Visibility::Public(self)
    }
}

impl IntoVisibility for VisRestricted {
    fn into_visibility(self) -> Visibility {
        Visibility::Restricted(self)
    }
}

pub fn visibility_public_variant() -> Visibility {
    Visibility::Public(Default::default())
}

const RESTRICTED_CRATE_IDENTS: &[&str] = &["self", "super", "crate"];

pub fn vis_restricted(path: impl IntoPath) -> VisRestricted {
    let path = path.into_path();
    let custom_path = path
        .get_ident()
        .is_some_and(|ident| !RESTRICTED_CRATE_IDENTS.contains(&ident.to_string().as_str()));

    VisRestricted {
        pub_token: Default::default(),
        paren_token: Default::default(),
        in_token: custom_path.then(Default::default),
        path: path.into(),
    }
}

pub fn field_mutability_none_variant() -> FieldMutability {
    FieldMutability::None
}
