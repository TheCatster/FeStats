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

    pub fn get_2_sample_t_interval(&mut self, x_bar_1: f64, sx_1: f64, n_1: f64, x_bar_2: f64, sx_2: f64, n_2: f64, c_level: f64) -> Result<String> {
        self.python.run(python! {
            from scipy.stats import t
            from math import sqrt

            n1 = 'n_1
            n2 = 'n_2
            m1 = 'x_bar_1
            m2 = 'x_bar_2

            v1 = 'sx_1
            v2 = 'sx_2
            pooled_se = sqrt(v1 / n1 + v2 / n2)
            delta = m1-m2

            tstat = delta / pooled_se
            df = (v1 / n1 + v2 / n2)**2 / (v1**2 / (n1**2 * (n1 - 1)) + v2**2 / (n2**2 * (n2 - 1)))

            # upper and lower bounds
            lower = delta - t.ppf(0.975,df)*pooled_se
            upper = delta + t.ppf(0.975,df)*pooled_se
        });

        let lower = self.python.get::<f64>("lower");
        let upper = self.python.get::<f64>("upper");

        Ok(format!("Lower Bound: {}\nUpper Bound: {}", lower, upper))
    }

    pub fn get_2_proportion_z_interval(&mut self, x_1: f64, n_1: f64, x_2: f64, n_2: f64, c_level: f64) -> Result<String> {
        self.python.run(python! {
            from scipy.stats import norm
            import numpy as np

            prop_a = 'x_1 / 'n_1
            prop_b = 'x_2 / 'n_2
            var = prop_a * (1 - prop_a) / 'n_1 + prop_b * (1 - prop_b) / 'n_2
            se = np.sqrt(var)

            confidence = 'c_level
            significance = 1 - 'c_level
            z = norm(loc = 0, scale = 1).ppf(confidence + significance / 2)

            prop_diff = prop_b - prop_a
            (lower, upper) = prop_diff + np.array([-1, 1]) * z * se
        });

        let lower = self.python.get::<f64>("lower");
        let upper = self.python.get::<f64>("upper");

        Ok(format!("Lower Bound: {}\nUpper Bound: {}", lower, upper))
    }
}
