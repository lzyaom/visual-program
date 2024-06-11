use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]

pub struct Plugin {
    name: String,
    url: String,
    descript: String,
    icon: String,
    // meta: Map<String, String>,
}

impl Plugin {
    /// Creates a new [`Plugin`].
    fn new(self, data: Plugin) -> Self {
        Plugin { ..data }
    }
}
