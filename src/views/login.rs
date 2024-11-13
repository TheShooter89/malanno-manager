use loco_rs::prelude::*;

/// Render app homepage
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn login(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "login/login.html", data!({}))
}
