use crate::app::App;
use anyhow::Result;
use inline_python::pyo3::types::PyTuple;
use inline_python::python;

impl<'a> App<'a> {
    pub fn get_z_interval(
        &mut self,
        sigma: f64,
        x_bar: f64,
        n: f64,
        c_level: f64,
    ) -> Result<String> {
        self.python.run(python! {
            from scipy.stats import norm
            from math import sqrt

            (lower, upper) = norm.interval('c_level, 'x_bar, 'sigma/sqrt('n))
        });

        let lower: f64 = self.python.get::<f64>("lower");
        let upper: f64 = self.python.get::<f64>("upper");

        Ok(format!("Lower Bound: {}\nUpper Bound: {}", lower, upper))
    }

    pub fn get_t_interval(&mut self, x_bar: f64, sx: f64, df: f64, c_level: f64) -> Result<String> {
        self.python.run(python! {
            from scipy.stats import t

            (lower, upper) = t.interval(alpha='c_level, df='df, loc='x_bar, scale='sx)
        });

        let lower = self.python.get::<f64>("lower");
        let upper = self.python.get::<f64>("upper");

        Ok(format!("Lower Bound: {}\nUpper Bound: {}", lower, upper))
    }
}
