pub mod api {
    pub mod v1 {
        pub mod core {
            include!(concat!(env!("OUT_DIR"), "/api.core.v1.rs"));
        }
    }
}

pub mod apimachinery {
    pub mod pkg {
        pub mod api {
            pub mod resource {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/apimachinery.pkg.api.resource.rs"
                ));
            }
        }

        pub mod apis {
            pub mod meta {
                pub mod v1 {
                    include!(concat!(
                        env!("OUT_DIR"),
                        "/apimachinery.pkg.apis.meta.v1.rs"
                    ));
                }
            }
        }

        pub mod runtime {
            include!(concat!(env!("OUT_DIR"), "/apimachinery.pkg.runtime.rs"));

            pub mod schema {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/apimachinery.pkg.runtime.schema.rs"
                ));
            }
        }

        pub mod util {
            pub mod intstr {
                include!(concat!(env!("OUT_DIR"), "/apimachinery.pkg.util.intstr.rs"));
            }
        }
    }
}
