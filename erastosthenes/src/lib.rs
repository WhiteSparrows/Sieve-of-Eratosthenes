pub fn naive(n: usize) -> Vec<usize>{
    let mut prime_numbers = Vec::new();
    let mut vec: Vec<usize> = (2..n).collect();
    let mut p = 2;
    while !vec.is_empty(){
        vec.retain(|&x| x%p != 0);
        if !vec.is_empty(){
            prime_numbers.push(p);
            p = vec[0]
        }

    }
    prime_numbers

}
    
pub fn optimize(n: usize) -> Vec<usize>{
    let mut prime_numbers = vec![true; n];
    let ceil  = (n as f64).sqrt() as usize;
    let mut j;
    for i in 2..ceil{
        if prime_numbers[i]{
            j = i*i;
            while j < n{
                prime_numbers[j] = false;
                j = j + i;
            }
        }
    }
    prime_numbers
        .iter()
        .enumerate()
        .filter_map(|(index, &element)| (element && index != 0 && index != 1).then(|| index))
        .collect::<Vec<_>>()
}
