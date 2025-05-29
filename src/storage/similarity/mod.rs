// https://en.wikipedia.org/wiki/Cosine_similarity
pub fn cosine_similarity(a: Vec<usize>, b: Vec<usize>) -> f64 {
    let mut sum_of_product = 0;
    let mut a_norm = 0;
    let mut b_norm = 0;
    for i in 0..a.len() {
        sum_of_product += a[i] * b[i];
        a_norm += a[i] * a[i];
        b_norm += b[i] * b[i];
    }

    (sum_of_product as f64) / (f64::sqrt(a_norm as f64) * f64::sqrt(b_norm as f64))
}
