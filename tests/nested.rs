use delegate::delegate;

#[derive(Clone)]
struct Inner;
impl Inner {
    pub fn method(&self, num: u32) -> u32 {
        num
    }
}

struct Inner2 {
    inner: Inner,
}

struct Wrapper {
    inner: Inner2,
}

impl Wrapper {
    delegate! {
        to self.inner.inner {
            pub(crate) fn method(&self, num: u32) -> u32;
        }
    }
    delegate! {
        to self.inner {
            #[field(&inner)]
            pub(crate) fn inner(&self) -> &Inner;
            #[field(&mut inner)]
            pub(crate) fn inner_mut(&mut self) -> &mut Inner;
        }
    }
}

#[test]
fn test_nested() {
    let wrapper = Wrapper {
        inner: Inner2 { inner: Inner },
    };

    assert_eq!(wrapper.method(3), 3);
    assert_eq!(wrapper.inner().method(4), 4);
}
