mod http{
    pub mod request{
        use super::method::Method;

        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: Method
        }
    }

    pub mod method{
        pub enum Method {
            GET, 
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH
        }
    }
}