pub mod cosmos {
    pub mod tx {
        pub mod signing {
            // @@protoc_insertion_point(attribute:cosmos.tx.signing.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.tx.signing.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.tx.signing.v1beta1)
            }
        }

        // @@protoc_insertion_point(attribute:cosmos.tx.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.tx.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.tx.v1beta1)
        }
    }
    pub mod upgrade {
        // @@protoc_insertion_point(attribute:cosmos.upgrade.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.upgrade.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.upgrade.v1beta1)
        }
    }

    pub mod gov {
        // @@protoc_insertion_point(attribute:cosmos.gov.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.gov.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.gov.v1beta1)
        }
        pub mod v1 {
            include!("cosmos.gov.v1.rs");
            // @@protoc_insertion_point(cosmos.gov.v1)
        }
    }

    pub mod base {
        // @@protoc_insertion_point(attribute:cosmos.base.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.base.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.base.v1beta1)
        }
    }

    pub mod crypto {
        pub mod multisig {
            // @@protoc_insertion_point(attribute:cosmos.crypto.multisig.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.crypto.multisig.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.crypto.multisig.v1beta1)
            }
        }
    }

    pub mod params {
        // @@protoc_insertion_point(attribute:cosmos.params.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.params.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.params.v1beta1)
        }
    }

    pub mod distribution {
        // @@protoc_insertion_point(attribute:cosmos.distribution.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.distribution.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.distribution.v1beta1)
        }
    }

    pub mod authz {
        // @@protoc_insertion_point(attribute:cosmos.authz.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.authz.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.authz.v1beta1)
        }
    }
}

pub mod ibc {
    pub mod core {
        pub mod client {
            pub mod v1 {
                include!("ibc.core.client.v1.rs");
            }
        }
    }
}
