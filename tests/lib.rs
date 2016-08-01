extern crate simple_rest;

#[cfg(test)]
mod tests {

    use simple_rest::Rest;
    use std::time::Duration;

    #[test]
    fn rest_get_single() {
        let api = Rest::Endpoint::configure()
            .url("http://jsonplaceholder.typicode.com")
            .timeout(Duration::from_secs(5))
            //.add_header("header", "value")
            .build();

        // EndpointBuilder
        // EndPoint

        //RequestBuilder
        //Request

        let request = Rest::Request::get()
            //.path("resource/{id}")
            //.add_parameter("key", "value")
            //.add_urlsegment("id", "123")
            .build();

        //Response
        let response = Rest::Client::execute(api, request);

    }


}