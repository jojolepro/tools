error_chain! {
    foreign_links {
        Io(::std::io::Error) #[doc = "IO error"];
        VersionReq(::semver::ReqParseError) #[doc = "Could not parse version requirements"];
        SemVer(::semver::SemVerError) #[doc = "Could not parse version"];
    }

    errors {
        New(name: String) {
            description("project creation failed")
            display("project creation for project {:?} failed", name)
        }

        /// Don't have a template matching this version of the `amethyst` crate
        UnsupportedVersion(version: String) {
            description("unsupported version of Amethyst requested")
            display("This version of amethyst_tools does not support the requested version {:?}", version)
        }

        UnknownTemplate(template_name: String) {
            description("Unknown template name requested")
            display("The template name you requested has not been found: {}. See amethyst list-templates.", template_name)
        }

        /// Failed to fetch `amethyst` crate version from crates.io
        FetchVersionFailure {
            description("Failed to fetch latest version of amethyst")
        }

        /// The fetched crates.io JSON is not valid
        InvalidCratesIoJson {
            description("The JSON fetched from crates.io is invalid")
        }
    }
}
