use foundry_binder::Binder;

fn main() {
    let binder = Binder::new("../contracts")
        .keep_artifacts(".artifacts");
    binder.generate().expect("Failed to generate bindings");
}
