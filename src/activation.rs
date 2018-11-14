pub fn sigmoid(x:f64) -> f64{
1.0/(1.0+(-x).exp())
}

fn tanh(x:f64) -> f64{
48.0
}

fn linear(x:f64) -> f64{
48.0
}

fn unit_step(x:f64) -> f64{
48.0
}

fn binary(x:f64) -> f64 {
48.0
}

#[cfg(test)]
mod tests {
use super::*;
#[test]
fn test_sigmoid() {
    assert_eq!(sigmoid(5.0),0.993307149075715144440638019618674819606255991092703469);
}

}



