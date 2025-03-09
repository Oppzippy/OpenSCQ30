use cosmic::app::context_drawer::ContextDrawer;

pub fn coalesce_result<T>(result: Result<T, T>) -> T {
    match result {
        Ok(ok) => ok,
        Err(err) => err,
    }
}

pub trait ContextDrawerMapExt<'a, Message>
where
    Message: 'a + Clone,
{
    fn map<B>(self, f: impl Fn(Message) -> B + Clone + 'a) -> ContextDrawer<'a, B>
    where
        B: 'a + Clone;
}

impl<'a, Message> ContextDrawerMapExt<'a, Message> for ContextDrawer<'a, Message>
where
    Message: Clone,
{
    fn map<B>(self, f: impl Fn(Message) -> B + Clone + 'a) -> ContextDrawer<'a, B>
    where
        Message: 'a,
        B: 'a + Clone,
    {
        ContextDrawer {
            title: self.title,
            header_actions: self
                .header_actions
                .into_iter()
                .map(|action| action.map(f.clone()))
                .collect(),
            header: self.header.map(|header| header.map(f.clone())),
            content: self.content.map(f.clone()),
            footer: self.footer.map(|footer| footer.map(f.clone())),
            on_close: f(self.on_close),
        }
    }
}
