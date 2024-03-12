use crate::pojo::Config;

pub struct DirTree<'a> {
    term: &'a mut Box<term::StdoutTerminal>,
    config: Config,
}

impl<'a> DirTree<'a> {
    pub fn new(config: Config, term: &'a mut Box<term::StdoutTerminal>) -> DirTree<'a> {
        DirTree {
            config,
            term,
        }
    }
}
