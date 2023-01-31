#[derive(Debug)]
pub struct LinearModel {
    x: Vec<f64>,
    y: Vec<f64>,
    b0: f64,
    b1: f64
}

impl LinearModel {
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> LinearModel {
        LinearModel {
            x: x,
            y: y,
            b0: 0.0,
            b1: 0.0
        }
    }

    pub fn fit(&mut self) -> Option<()> {
        let mut x_mean:f64 = self.x.iter().sum();
        let mut y_mean:f64 = self.y.iter().sum();

        x_mean /= self.x.len() as f64;
        y_mean /= self.y.len() as f64;
        

        self.b1 = self.x.iter().zip(self.y.iter()).map(|(x, y)| (x - x_mean) * (y - y_mean)).sum::<f64>() / self.x.iter().map(|x| (x - x_mean) * (x - x_mean)).sum::<f64>();
        self.b0 = y_mean - self.b1 * x_mean;

        Some(())
    }

    pub fn predict(&self, x: f64) -> f64 {
        self.b0 + self.b1 * x
    }
}