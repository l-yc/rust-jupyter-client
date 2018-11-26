use header::Header;
use metadata::Metadata;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct HelpLink {
    pub text: String,
    pub url: String,
}

#[derive(Debug)]
pub enum Response {
    KernelInfo {
        header: Header,
        parent_header: Header,
        metadata: Metadata,
        content: KernelInfoContent,
    },
}

#[derive(Deserialize, Debug)]
pub struct KernelInfoContent {
    pub banner: String,
    pub implementation: String,
    pub implementation_version: String,
    pub protocol_version: String,
    pub status: String,
    pub help_links: Vec<HelpLink>,
}
