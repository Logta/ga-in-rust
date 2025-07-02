/// Placeholder for future API interface
use crate::core::errors::GAResult;

/// REST API interface (future implementation)
pub struct ApiServer {
    port: u16,
}

impl ApiServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn start(&self) -> GAResult<()> {
        // Future implementation
        println!("API server would start on port {}", self.port);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_server_creation() {
        let server = ApiServer::new(8080);
        assert_eq!(server.port, 8080);
    }

    #[test]
    fn test_api_server_start() {
        let server = ApiServer::new(3000);
        let result = server.start();
        assert!(result.is_ok());
    }
}
