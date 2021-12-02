use maths_common::{
    proto::{
        add::add_client::AddClient,
        maths::{AddRequest, MulRequest, MulResponse},
        mul::mul_server::Mul,
    },
    set_trace_ctx, trace_req,
};
use tonic::{transport::Channel, Request, Response, Status};
use tracing::*;

#[derive(Debug)]
pub struct MulService {
    add_client: AddClient<Channel>,
}

impl MulService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let add_client = AddClient::connect("http://127.0.0.1:12341").await?;
        Ok(Self { add_client })
    }

    #[instrument]
    pub async fn mul_two_values(&self, x: i32, y: i32) -> Result<i64, Box<dyn std::error::Error>> {
        // Super secret-squirrel multiplication algorithm
        let mut acc = 0;
        for _ in 0..y {
            let req = trace_req(AddRequest {
                val1: acc as i32,
                val2: x,
            });
            let resp = self
                .add_client
                .clone()
                .add(req)
                .instrument(info_span!("rpc: call add-serivce add"))
                .await?
                .into_inner();
            acc = resp.result;
        }

        Ok(acc)
    }
}

#[tonic::async_trait]
impl Mul for MulService {
    #[instrument]
    async fn mul(&self, req: Request<MulRequest>) -> Result<Response<MulResponse>, Status> {
        set_trace_ctx(&req);
        let req = req.into_inner();
        Ok(Response::new(MulResponse {
            result: self
                .mul_two_values(req.val1, req.val2)
                .await
                .map_err(|e| Status::internal(e.to_string()))?,
        }))
    }
}
