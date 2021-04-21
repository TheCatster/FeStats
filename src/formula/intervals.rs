use crate::app::App;
use inline_python::python;

impl<'a> App<'a> {
    pub fn test(&mut self, n: f64) -> String {
        self.python.run(python! {
            z = 'n + 12
        });

        format!("{}", self.python.get::<f64>("z"))
    }
}