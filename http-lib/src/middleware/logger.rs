use crate::middleware::middleware::FutureRequest;
use crate::middleware::middleware::Middleware;
use crate::request::Request;
use crate::response::Response;
use crate::server::FutureResponse;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct LoggerMiddleware;

impl Middleware for LoggerMiddleware {
    fn on_request<'a>(&self, request: Request) -> FutureRequest<'a> {
        // Log the incoming request
        println!("Request HERE: {:?}", request);

        Box::pin(async move {
            // Modify the request or perform asynchronous operations
            Ok(request)
            // Modify as per your implementation
        })
    }

    fn on_response<'a>(&self, response: Response) -> FutureResponse<'a> {
        // Log the outgoing response
        println!("Response: {:?}", response);
        Box::pin(async move {
            Ok(response) // Wrap response.clone() in Ok variant
        })
    }
}
