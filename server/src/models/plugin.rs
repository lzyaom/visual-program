use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]

pub struct Plugin {
    pub name: String,
    pub url: String,
    pub descript: String,
    pub icon: String,
    // meta: Map<String, String>,
}

impl Plugin {
    /// Creates a new [`Plugin`].
    fn _new(self, data: Plugin) -> Self {
        Plugin { ..data }
    }
}
