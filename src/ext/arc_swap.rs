use arc_swap::ArcSwapOption;
use std::sync::Arc;

pub(crate) trait ArcSwapOptionExt<T> {
    fn store_value(&self, value: T);
}

impl<T> ArcSwapOptionExt<T> for ArcSwapOption<T> {
    #[inline(always)]
    fn store_value(&self, value: T) {
        self.store(Some(Arc::new(value)));
    }
}
