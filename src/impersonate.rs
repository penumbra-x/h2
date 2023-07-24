use std::cell::RefCell;

pub enum ClientType {
    Chrome,
    OkHttp
}

impl ClientType {

    thread_local! {
        pub static T: RefCell<Option<ClientType>> = RefCell::new(None);
    }

    pub fn set_thread_local(client_type: ClientType) {
        Self::T.with(|v| {
            *v.borrow_mut() = Some(client_type)
        })
    }

    pub fn get_thread_local() -> Option<ClientType> {
        Self::T.with(|var| {
            var.borrow().clone()
        })
    }

}