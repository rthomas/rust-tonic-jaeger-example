use maths_common::set_trace_ctx;
use maths_common::{
    proto::{
        add::add_client::AddClient,
        maths::{maths_server::Maths, AddRequest, AddResponse, MulRequest, MulResponse},
        mul::mul_client::MulClient,
    },
    trace_req,
};
use tonic::{transport::Channel, Request, Response, Status};
use tracing::*;

#[derive(Debug)]
pub struct MathsService {
    add_client: AddClient<Channel>,
    mul_client: MulClient<Channel>,
}

impl MathsService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let add_client = AddClient::connect("http://127.0.0.1:12341").await?;
        let mul_client = MulClient::connect("http://127.0.0.1:12342").await?;

        Ok(Self {
            add_client,
            mul_client,
        })
    }
}

#[tonic::async_trait]
impl Maths for MathsService {
    #[instrument(name = "RPC: add")]
    async fn add(&self, req: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        set_trace_ctx(&req);
        let req = trace_req(req.into_inner());
        self.add_client
            .clone()
            .add(req)
            .instrument(info_span!("rpc: call add-service add"))
            .await
    }

    #[instrument(name = "RPC: mul")]
    async fn mul(&self, req: Request<MulRequest>) -> Result<Response<MulResponse>, Status> {
        set_trace_ctx(&req);
        let req = trace_req(req.into_inner());
        self.mul_client
            .clone()
            .mul(req)
            .instrument(info_span!("rpc: call mul-service mul"))
            .await
    }
}
