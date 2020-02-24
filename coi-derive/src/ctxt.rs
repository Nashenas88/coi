use std::cell::RefCell;

pub struct Ctxt {
    errors: RefCell<Vec<syn::Error>>,
}

impl Ctxt {
    pub fn new() -> Self {
        Ctxt { errors: RefCell::new(vec![]) }
    }

    pub fn push(&self, err: syn::Error) {
        self.errors.borrow_mut().push(err)
    }

    pub fn check(self) -> Result<(), Vec<syn::Error>> {
        if self.errors.borrow().is_empty() {
            Ok(())
        } else {
            Err(self.errors.into_inner())
        }
    }
}
