mod threads;

pub struct Server {
    threads: Option<threads::ThreadManager>
}
