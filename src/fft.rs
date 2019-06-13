use num_complex::Complex;
use chfft::RFft1D;

pub fn find_frequencies(buffer: &[f32], sample_rate: f32, magnitude_cutoff: f32) -> Vec<f32> {
    let fft_output = fft(buffer);
    let bin_step = sample_rate / buffer.len() as f32;

    fft_output
        .iter()
        .map(|x| f32::sqrt(x.norm_sqr()))
        .enumerate()
        .filter(|x| x.1 > magnitude_cutoff)
        .map(|x| x.0 )
        .map(|x| {
            let k =  quinns_second_interpolate(&fft_output, x);
            k * bin_step
        })
        .collect()
}

pub fn fft(buffer: &[f32]) -> Vec<Complex<f32>> {
    let input: Vec<f32> = buffer.iter().map(|x| *x).collect();
    let mut fft =  RFft1D::<f32>::new(input.len());
    fft.forward(input.as_slice())
}

/// More expensive, but more accurate interpolation for fourier analysis.
fn quinns_second_interpolate(fft_output: &[Complex<f32>], peak_index: usize ) -> f32 {
    fn tau(x: f32) -> f32 {
        1.0/4.0 * f32::log10((3.0*x).powi(2) + (6.0 * x) + 1.0) - f32::sqrt(6.0)/24.0 * f32::log10((x + 1.0 - f32::sqrt(2.0/3.0))  /  (x + 1.0 + f32::sqrt(2.0/3.0)))
    }
    let o = fft_output;
    let k = peak_index;
    let ap: f32 = (o[k + 1].re * o[k].re + o[k + 1].im * o[k].im) / (o[k].re.powi(2) + o[k].im.powi(2));
    let dp: f32 = -ap / (1.0 - ap);

    let am: f32 = {
        let o_at_k_minus_one = o.get(k - 1).cloned().unwrap_or_default();
        (o_at_k_minus_one.re * o[k].re + o_at_k_minus_one.im * o[k].im) / (o[k].re.powi(2) + o[k].im.powi(2))
    };
    let dm: f32 = am / (1.0 - am);
    let d = (dp + dm) / 2.0 + tau(dp * dp) - tau(dm * dm);
    k as f32 + d
}
