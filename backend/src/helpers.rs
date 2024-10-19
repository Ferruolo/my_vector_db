use tch::Tensor;

pub(crate) fn binary_search(
    indexes: &Vec<Tensor>,
    query: &Tensor,
    compare: &impl Fn(&Tensor, &Tensor) -> bool,
) -> usize {
    let mut low: usize = 0;
    let mut high: usize = indexes.len();
    let mut mid: usize;
    while low < high {
        mid = low + ((high - low) / 2);
        if compare(&indexes[mid], query) {
            low = mid
        } else {
            high = mid;
        }
    }
    low
}

pub(crate) fn cosine_similarity_rust_float(l: &Tensor, r: &Tensor) -> f32 {
    let dot_product = l.dot(r);
    let l_norm = l.norm();
    let r_norm = r.norm();
    let similarity = dot_product / (l_norm * r_norm);
    similarity.double_value(&[]) as f32
}

pub(crate) fn binary_search_floats(array: &Vec<f32>, query: &f32) -> usize {
    let mut low = 0;
    let mut high = array.len();
    let mut mid: usize;
    while low < high {
        mid = low + (high - low) / 2;
        if array[mid] < *query {
            high = mid;
        } else {
            low = mid;
        }
    }
    low
}

pub(crate) fn compare(zero: &Tensor) -> impl Fn(&Tensor, &Tensor) -> bool {
    move |l: &Tensor, r: &Tensor| {
        cosine_similarity_rust_float(l, zero) < cosine_similarity_rust_float(r, zero)
    }
}
