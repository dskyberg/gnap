
const config = [
    {
        "service_endpoints": {
            grant_request_endpoint: "localhost::8000/gnap/tx",
            introspection_endpoint: "https://localhost:8000/gnap/introspect",
            resource_registration_endpoint: "https://localhost:8000/gnap/resource",
        }
    },

    {
        "interaction_start_modes_supported": [
            "redirect",
            "app",
            "user_code"
        ]
    },

    {
        "interaction_finish_methods_supported": [
            "redirect",
            "push"
        ]
    },
    {
        "key_proofs_supported": [
            "httpsig",
            "mtls",
            "jwsd",
            "jws"
        ]
    },
    {
        "subject_formats_supported": [
            "account",
            "aliases",
            "did",
            "email",
            "iss_sub",
            "opaque",
            "phone_number"
        ]
    },
    {
        "assertions_supported": [
            "oidc",
            "saml2"
        ]
    }
];

conn = new Mongo();
db = conn.getDB("gnap");
db.service_config.insertMany(config);



