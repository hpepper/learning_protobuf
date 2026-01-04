// Bring in the generated protobuf code
mod connection {
    include!("../proto_gen/generated.rs");
}

use connection::{ConnectionRequest, ConnectionResponse};

fn server(req: ConnectionRequest) -> ConnectionResponse {
    let mut response = ConnectionResponse::default();
    if req.certificate().is_empty() {
        response.set_return_code(1);
        response.set_failure_reason("Certificate is missing".to_string());
        response
    } else {
        response.set_return_code(0);
        response.set_failure_reason("".to_string());
        response
    }
}

fn main() {
    // Create a test request
    let mut request = ConnectionRequest::default();
    request.set_certificate(b"dummy-certificate-bytes".to_vec());

    // Call the server function
    let response = server(request);

    // println!("Return code: {}", response.return_code);

    if response.failure_reason().is_empty() {
        println!("Connection successful");
    } else {
        println!("Failure reason: {}", response.failure_reason());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_with_valid_certificate() {
        let mut request = ConnectionRequest::default();
        request.set_certificate(b"valid-certificate".to_vec());

        let response = server(request);

        assert_eq!(response.return_code(), 0);
        assert_eq!(response.failure_reason(), "");
    }

    #[test]
    fn test_server_with_empty_certificate() {
        let request = ConnectionRequest::default();

        let response = server(request);

        assert_eq!(response.return_code(), 1);
        assert_eq!(response.failure_reason(), "Certificate is missing");
    }

    #[test]
    fn test_server_with_small_certificate() {
        let mut request = ConnectionRequest::default();
        request.set_certificate(b"x".to_vec());

        let response = server(request);

        assert_eq!(response.return_code(), 0);
        assert_eq!(response.failure_reason(), "");
    }

    #[test]
    fn test_server_with_large_certificate() {
        let mut request = ConnectionRequest::default();
        let large_cert = vec![0u8; 1024];
        request.set_certificate(large_cert);

        let response = server(request);

        assert_eq!(response.return_code(), 0);
        assert_eq!(response.failure_reason(), "");
    }

    #[test]
    fn test_connection_request_default() {
        let request = ConnectionRequest::default();
        assert!(request.certificate().is_empty());
    }

    #[test]
    fn test_connection_response_default() {
        let response = ConnectionResponse::default();
        assert_eq!(response.return_code(), 0);
        assert_eq!(response.failure_reason(), "");
    }
}