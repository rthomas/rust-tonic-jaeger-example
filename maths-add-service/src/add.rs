use maths_common::{
    proto::{
        add::add_server::Add,
        maths::{AddRequest, AddResponse},
    },
    set_trace_ctx,
};
use tonic::{Request, Response, Status};
use tracing::*;

#[derive(Debug)]
pub struct AddService;

impl AddService {
    #[instrument]
    pub fn add_two_values(&self, x: i32, y: i32) -> i64 {
        x as i64 + y as i64
    }
}

#[tonic::async_trait]
impl Add for AddService {
    #[instrument]
    async fn add(&self, req: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        set_trace_ctx(&req);
        let req = req.into_inner();
        Ok(Response::new(AddResponse {
            result: self.add_two_values(req.val1, req.val2),
        }))
    }
}
