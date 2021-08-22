use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct GeneratorState {
    service_names: Vec<String>,
    package_names: Vec<String>,
    finalized: usize,
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
    fn generate(&mut self, service: prost_build::Service, _buf: &mut String) {
        let mut state = self.state.borrow_mut();
        state.service_names.push(service.name);
    }

    fn finalize(&mut self, _buf: &mut String) {
        let mut state = self.state.borrow_mut();
        state.finalized += 1;
    }

    fn finalize_package(&mut self, package: &str, buf: &mut String) {
        let mut state = self.state.borrow_mut();
        state.package_names.push(package.to_string());
        // TODO: generate generics for pkg here using self.data
        let pkg_generics = format!("// blahtest");
        buf.push_str(&pkg_generics);
    }
}

fn main() -> std::io::Result<()> {
    let protos: Vec<&str> = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/protos.list"))
        .lines()
        .collect();

    let state = Rc::new(RefCell::new(GeneratorState::default()));
    prost_build::Config::new()
        .service_generator(Box::new(KubeGenerator::new(Rc::clone(&state))))
        .compile_protos(protos.as_slice(), &["protos/"])?;

    // sanity
    let state = state.borrow();
    //assert_eq!(state.service_names.len(), protos.len()); zero atm..
    assert_eq!(state.finalized, protos.len());

    // Generate code in `src/` by reading files in `OUT_DIR`?
    // Need to create `mod.rs` file for each level based on the filename, and write generated code in correct file.
    Ok(())
}
