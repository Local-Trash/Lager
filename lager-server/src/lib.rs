mod threads;

pub struct Server {
    thread_pool: Option<threads::ThreadPool>,

}

pub trait Requester: Send {
    type RequestType: Send;

    fn request_receive(&self, request: Self::RequestType);
}

pub enum HTTPRequest {
    Api {},
    Page {}
}