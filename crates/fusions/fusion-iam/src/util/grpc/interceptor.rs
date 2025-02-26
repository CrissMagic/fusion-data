use tonic::{Request, Status};

use fusiondata_context::ctx::CtxW;

pub fn auth_interceptor(mut request: Request<()>) -> Result<Request<()>, Status> {
  let ctx: CtxW = CtxW::try_from(request.metadata())?;
  request.extensions_mut().insert(ctx);

  Ok(request)
}
