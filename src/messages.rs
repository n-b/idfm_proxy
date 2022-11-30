use std::sync::Arc;

use actix::prelude::*;
use siri_lite::service_delivery::EstimatedVehicleJourney;
use crate::status::Status;


#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<DataUpdate>,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct SiriUpdate {
    pub vjs: Vec<EstimatedVehicleJourney>,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct DataUpdate {
    pub pt_data: Arc<crate::PTData>,
}

#[derive(Message, Clone)]
#[rtype(result = "Arc<Status>")]
pub struct StatusDemand {
}
