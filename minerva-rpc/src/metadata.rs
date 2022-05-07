use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::service::Interceptor;
use tonic::{Request, Status};

pub fn get_value(map: &MetadataMap, key: &str) -> Option<String> {
    map.get(key).map(|v| v.to_str().unwrap_or("unknown").into())
}

pub fn push_metadata(
    request: &mut Request<()>,
    tenant: &str,
    requestor: &str,
) -> Result<(), String> {
    request.metadata_mut().insert(
        "tenant",
        MetadataValue::from_str(tenant)
            .map_err(|e| format!("Failure while applying tenant to request: {}", e))?,
    );

    request.metadata_mut().insert(
        "requestor",
        MetadataValue::from_str(requestor)
            .map_err(|e| format!("Failure while applying requestor to request: {}", e))?,
    );

    Ok(())
}

pub struct ClientInterceptor {
    tenant: String,
    requestor: String,
}

impl ClientInterceptor {
    pub fn new(tenant: &str, requestor: &str) -> Self {
        Self {
            tenant: tenant.into(),
            requestor: requestor.into(),
        }
    }
}

impl Interceptor for ClientInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        push_metadata(&mut request, self.tenant.as_ref(), self.requestor.as_ref())
            .map_err(|e| Status::failed_precondition(e))?;
        Ok(request)
    }
}