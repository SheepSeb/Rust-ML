mod linear_model;
use crate::linear_model::LinearModel;
fn main() {
    // Test for liniar model
    let x = vec![2.0, 4.0, 3.0, 6.0, 10.0];
    let y = vec![1.0, 2.01, 1.52, 3.01, 5.20];

    let mut model = LinearModel::new(x, y);
    model.fit();

    let test_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let y_pred = test_data.iter().map(|x| model.predict(*x));

    for y in y_pred {
        println!("{}", y);
    }
}
