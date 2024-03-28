pub trait BunkResponse<TResponse> {
    fn get(&self) -> TResponse;
}

pub trait BunkRequest<TResponse> {
    fn send(&self) -> dyn BunkResponse<TResponse>;
}
