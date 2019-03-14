use std::marker::PhantomData;

#[derive(Debug)]
pub struct RootType<T> {
    pub format: String,
    system_type: PhantomData<T>,
}

impl<T> RootType<T> {
    pub fn new(fmt: &str) -> RootType<T> {
        RootType {
            format: fmt.to_string(),
            system_type: PhantomData,
        }
    }

    pub fn set_format(&mut self, fmt: &str) {
        self.format = fmt.to_string();
    }

    pub fn get_format(&self) -> &str {
        &self.format
    }
}
