use salvo::{handler, Depot, FlowCtrl, Request, Response};
use salvo::http::StatusCode;
use salvo::prelude::{Json, Text};
use serde::{Deserialize, Serialize};
use crate::config;
use crate::singleton::{CommandManager, COMMAND_MANAGER};

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
#[derive(Debug, Serialize, Deserialize)]
struct CommandStatus {
    pub uuid: String,
    pub status: i32,
    pub owner: String,
    pub start_time: String,
    pub end_time: String,
}

#[handler]
pub async fn get_status(res: &mut Response) {
    let mut status_list = Vec::new();
    for (_, command) in COMMAND_MANAGER.lock().await.command_map.iter() {
        let command = command.read().await;
        let status = CommandStatus {
            uuid: command.uuid.clone(),
            status: command.status.clone(),
            owner: command.command_owner.clone(),
            start_time: command.start_time.clone(),
            end_time: command.end_time.clone(),
        };
        status_list.push(status);
    }
    status_list.sort_by(|a, b| a.start_time.cmp(&b.start_time));
    res.render(Json(status_list));
}