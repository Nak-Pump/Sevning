use salvo::{handler, Depot, FlowCtrl, Request, Response};
use salvo::http::StatusCode;
use salvo::prelude::Text;
use crate::config;

#[handler]
pub async fn admin_guard(req: &mut Request, res: &mut Response, depot: &mut Depot, ctrl: &mut FlowCtrl) {
    let mut config = config::SConfig::new();
    config.init();
    let mut config = config.config;
    let password = req.query("password").unwrap_or("default").to_string();
    if config.application.verify == true {
        if password == config.application.password {
            ctrl.call_next(req, depot, res).await;
        } else {
            res.status_code(StatusCode::FORBIDDEN);
            res.render(Text::Plain("You are not authorized to access this resource."));
        }
    } else {
        ctrl.call_next(req, depot, res).await;
    }
}