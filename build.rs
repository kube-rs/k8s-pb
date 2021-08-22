use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct GeneratorState {
    service_names: Vec<String>,
    finalized: usize,
    generated: usize
}

struct KubeGenerator {
    data: String,
    state: Rc<RefCell<GeneratorState>>,
}
impl KubeGenerator {
    fn new(state: Rc<RefCell<GeneratorState>>) -> Self {
        let data = std::fs::read_to_string("./openapi/api-resources.json").unwrap();
        Self { data, state }
    }
}

impl prost_build::ServiceGenerator for KubeGenerator {
    fn generate(&mut self, service: prost_build::Service, buf: &mut String) {
        let mut state = self.state.borrow_mut();
        state.service_names.push(service.name);
        state.generated += 1;
        // TODO: THIS doesn't work? never called by prost_build, bug?
        let generics = format!("// TODO: generate\n");
        buf.push_str(&generics);
    }

    fn finalize(&mut self, buf: &mut String) {
        let mut state = self.state.borrow_mut();
        state.finalized += 1;
        // NB: THIS works, but we need a name here before it's useful
        //let generics = format!("// TODO: finalize\n");
        //buf.push_str(&generics);
    }
}

fn main() -> std::io::Result<()> {
    let protos: Vec<&str> = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/protos.list"))
        .lines()
        .collect();

    let state = Rc::new(RefCell::new(GeneratorState::default()));
    prost_build::Config::new()
        .service_generator(Box::new(KubeGenerator::new(Rc::clone(&state))))
        .out_dir("./out")
        .compile_protos(protos.as_slice(), &["protos/"])?;

    // sanity
    let state = state.borrow();
    assert_eq!(state.finalized, protos.len());
    assert_eq!(state.generated, protos.len()); // TODO: why does generate not trigger

    // Generate code in `src/` by reading files in `OUT_DIR`?
    // Need to create `mod.rs` file for each level based on the filename, and write generated code in correct file.
    Ok(())
}
