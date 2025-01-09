use gtk::prelude::*;

pub trait WalkChildren {
    fn walk_children(&self) -> impl Iterator<Item = gtk::Widget>;
}

impl<T: IsA<gtk::Widget>> WalkChildren for T {
    fn walk_children(&self) -> impl Iterator<Item = gtk::Widget> {
        GtkWidgetChildWalker::new(self)
    }
}

pub struct GtkWidgetChildWalker<'a, T: IsA<gtk::Widget>> {
    root: &'a T,
    current: Option<gtk::Widget>,
}

impl<'a, T: IsA<gtk::Widget>> GtkWidgetChildWalker<'a, T> {
    pub fn new(root: &'a T) -> Self {
        Self {
            current: root.first_child(),
            root,
        }
    }
}

impl<T: IsA<gtk::Widget>> Iterator for GtkWidgetChildWalker<'_, T> {
    type Item = gtk::Widget;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.current.to_owned()?;

        // First visit child node if possible
        if let Some(child) = current.first_child() {
            self.current = Some(child.to_owned());
            return Some(child);
        }

        // Otherwise, go to the next node
        if let Some(next) = current.next_sibling() {
            self.current = Some(next.to_owned());
            return Some(next);
        }

        // No more next nodes, so go up to the parent, which we visited already, and visit the parent's next sibling
        // Repeat if the parent has no next sibling
        loop {
            let parent = current.parent()?;
            if &parent == self.root {
                return None;
            }
            // we already visited parent, so advance one
            if let Some(next) = parent.next_sibling() {
                self.current = Some(next.to_owned());
                return Some(next);
            };
            current = parent;
        }
    }
}
