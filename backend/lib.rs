use ic_cdk::query;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[query]
fn pick_winner(names: Vec<String>) -> String {
    if names.is_empty() {
        return "No names provided.".to_string();
    }

    let seed = ic_cdk::api::time(); // nanoseconds
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let index = rng.gen_range(0..names.len());

    names[index].clone()
}
ic_cdk::export_candid!();





