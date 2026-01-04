// Bring in the generated protobuf code
mod edge {
    include!("../proto_gen/generated.rs");
}

use edge::{ConnectionRequest, ConnectionResponse, DisconnectionRequest, DisconnectionResponse, ConnectionRequestView, DisconnectionRequestView};


fn main() {
    // test with a request with a valid example with a dummy certificate.
    let mut connection_request = ConnectionRequest::default();
    connection_request.set_certificate(b"dummy-certificate-bytes".to_vec());

    let mut client_msg = edge::ClientMessage::default();
    client_msg.set_connection_request(connection_request);
    
    let response = server(client_msg);
    
    let sessionid = if let Some(conn_resp) = response.connection_response_opt().into_option() {
        if conn_resp.return_code() == 0 {
            println!("Connection successful");
            Some(conn_resp.session_id())
        } else {
            println!("Connection failure: {}", conn_resp.failure_reason());
            None
        }
    } else {
        println!("No connection response received");
        None
    };

    if sessionid.is_none() {
        return;
    }

    // Test disconnection request, re-use the session ID obtained from connection.
    let mut disconnection_request = DisconnectionRequest::default();
    disconnection_request.set_session_id(sessionid.unwrap());

    let mut client_msg2 = edge::ClientMessage::default();
    client_msg2.set_disconnection_request(disconnection_request);
    
    let response2 = server(client_msg2);
    
    if let Some(disc_resp) = response2.disconnection_response_opt().into_option() {
        if disc_resp.failure_reason().is_empty() {
            println!("Disconnection successful for session: {}", disc_resp.session_id());
        } else {
            println!("Disconnection failure for session {}: {}", 
                     disc_resp.session_id(), disc_resp.failure_reason());
        }
    }
}

fn server(msg: edge::ClientMessage) -> edge::ServerMessage {
    match msg.payload() {
        edge::client_message::PayloadOneof::ConnectionRequest(req) => {
            let resp = handle_connection_request(&req);
            let mut server_msg = edge::ServerMessage::default();
            server_msg.set_connection_response(resp);
            server_msg
        },
        edge::client_message::PayloadOneof::DisconnectionRequest(req) => {
            let resp = handle_disconnection_request(&req);
            let mut server_msg = edge::ServerMessage::default();
            server_msg.set_disconnection_response(resp);
            server_msg
        },
        edge::client_message::PayloadOneof::not_set(_) => {
            // Handle case where no payload is set
            let mut response = ConnectionResponse::default();
            response.set_return_code(1);
            response.set_failure_reason("No message payload set".to_string());
            let mut server_msg = edge::ServerMessage::default();
            server_msg.set_connection_response(response);
            server_msg
        }
    }
}

fn handle_connection_request(msg: &ConnectionRequestView) -> ConnectionResponse {
    let mut response = ConnectionResponse::default();
    if msg.certificate().is_empty() {
        response.set_return_code(1);
        response.set_failure_reason("Certificate is missing".to_string());
        response
    } else {
        response.set_return_code(0);
        response.set_failure_reason("".to_string());
        response.set_session_id(42);
        response
    }
}

fn handle_disconnection_request(msg: &DisconnectionRequestView) -> DisconnectionResponse {
    let mut response = DisconnectionResponse::default();
    response.set_session_id(msg.session_id());
    
    if msg.session_id() == 0 {
        response.set_return_code(1);
        response.set_failure_reason("Invalid session ID".to_string());
    } else {
        response.set_return_code(0);
        response.set_failure_reason("".to_string());
    }
    
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_with_valid_certificate() {
        let mut request = ConnectionRequest::default();
        request.set_certificate(b"valid-certificate".to_vec());
        
        let mut client_msg = edge::ClientMessage::default();
        client_msg.set_connection_request(request);

        let response = server(client_msg);
        
        let conn_resp = response.connection_response_opt().into_option().unwrap();
        assert_eq!(conn_resp.failure_reason(), "");
        assert_eq!(conn_resp.return_code(), 0);
        assert_eq!(conn_resp.session_id(), 42);
    }

    #[test]
    fn test_server_with_empty_certificate() {
        let request = ConnectionRequest::default();
        
        let mut client_msg = edge::ClientMessage::default();
        client_msg.set_connection_request(request);

        let response = server(client_msg);

        let conn_resp = response.connection_response_opt().into_option().unwrap();
        assert_eq!(conn_resp.return_code(), 1);
        assert_eq!(conn_resp.failure_reason(), "Certificate is missing");
    }

    #[test]
    fn test_server_with_small_certificate() {
        let mut request = ConnectionRequest::default();
        request.set_certificate(b"x".to_vec());
        
        let mut client_msg = edge::ClientMessage::default();
        client_msg.set_connection_request(request);

        let response = server(client_msg);

        let conn_resp = response.connection_response_opt().into_option().unwrap();
        assert_eq!(conn_resp.return_code(), 0);
        assert_eq!(conn_resp.failure_reason(), "");
    }

    #[test]
    fn test_server_with_large_certificate() {
        let mut request = ConnectionRequest::default();
        let large_cert = vec![0u8; 1024];
        request.set_certificate(large_cert);
        
        let mut client_msg = edge::ClientMessage::default();
        client_msg.set_connection_request(request);

        let response = server(client_msg);

        let conn_resp = response.connection_response_opt().into_option().unwrap();
        assert_eq!(conn_resp.return_code(), 0);
        assert_eq!(conn_resp.failure_reason(), "");
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

    #[test]
    fn test_disconnection_with_valid_session_id() {
        let mut request = DisconnectionRequest::default();
        request.set_session_id(12345);
        
        let mut client_msg = edge::ClientMessage::default();
        client_msg.set_disconnection_request(request);

        let response = server(client_msg);

        let disc_resp = response.disconnection_response_opt().into_option().unwrap();
        assert_eq!(disc_resp.session_id(), 12345);
        assert_eq!(disc_resp.return_code(), 0);
        assert_eq!(disc_resp.failure_reason(), "");
    }

    #[test]
    fn test_disconnection_with_invalid_session_id() {
        let request = DisconnectionRequest::default(); // session_id defaults to 0
        
        let mut client_msg = edge::ClientMessage::default();
        client_msg.set_disconnection_request(request);

        let response = server(client_msg);

        let disc_resp = response.disconnection_response_opt().into_option().unwrap();
        assert_eq!(disc_resp.session_id(), 0);
        assert_eq!(disc_resp.return_code(), 1);
        assert_eq!(disc_resp.failure_reason(), "Invalid session ID");
    }

    #[test]
    fn test_server_with_no_payload() {
        let client_msg = edge::ClientMessage::default(); // No payload set

        let response = server(client_msg);

        let conn_resp = response.connection_response_opt().into_option().unwrap();
        assert_eq!(conn_resp.return_code(), 1);
        assert_eq!(conn_resp.failure_reason(), "No message payload set");
    }
}