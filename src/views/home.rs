use loco_rs::prelude::*;

/// Render app homepage
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn homepage(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "home/homepage.html", data!({}))
}
