use crate::Args;

pub struct Messenger {
    args: Args
}

impl Messenger {
    pub fn new(args: Args) -> Messenger {
        Messenger { args }
    }

    pub fn debug(&self, formatted_string: String) {
        if self.args.silent { return }

        if self.args.debug {
            println!("{}", formatted_string);
        }
    }

    pub fn stats(&self, formatted_string: String) {
        if self.args.silent { return }

        if !self.args.no_stats {
            println!("{}", formatted_string);
        }
    }

    pub fn silent(&self, formatted_string: String) {
        if self.args.silent {
            println!("{}", formatted_string);
        }
    }

}
