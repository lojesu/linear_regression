pub fn teta_one(kms: &Vec<f32>, prices: &Vec<f32>) -> f32 {
    let kms_avg = avg(kms);
    let prices_avg = avg(prices);
    let numerator = numerator(kms, prices, kms_avg, prices_avg);
    let denominator = denominator(kms, kms_avg);
    numerator / denominator
}


pub fn teta_zero(kms: &Vec<f32>, prices: &Vec<f32>, teta_one: f32) -> f32 {
    avg(prices) - teta_one * avg(kms)
}


fn numerator(kms: &Vec<f32>, prices: &Vec<f32>, kms_avg: f32, prices_avg: f32) -> f32 {
    let kms_cov: Vec<f32> = kms.into_iter().map(|x| x - kms_avg).collect();
    let prices_cov: Vec<f32> = prices.into_iter().map(|x| x - prices_avg).collect();
    let mut numerator = 0.0;
    let mut i = 0;
    while i < kms_cov.len() {
        numerator += kms_cov[i] * prices_cov[i];
        i += 1;
    }
    return numerator
}


fn denominator(kms: &Vec<f32>, kms_avg: f32) -> f32 {
    let kms_var: Vec<f32> = kms.into_iter().map(|x| x.powi(2) - kms_avg.powi(2)).collect();
    let mut denominator = 0.0;
    for x in kms_var.iter() {
        denominator += x;
    }
    return denominator
}


fn avg(v: &Vec<f32>) -> f32 {
    let mut avg = 0.0;
    for x in v.iter() {
        avg += x;
    }
    avg / v.len() as f32
}
