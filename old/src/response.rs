#[derive(Debug)]
pub enum Response {
    KernelInfoResponse(KernelInfoResponseDetails),
    StatusResponse(StatusResponseDetails),
}

#[derive(Deserialize, Debug)]
pub struct KernelInfoResponseDetails {
    content: KernelInfoContent,
    header: Header,
    metadata: Metadata,
    parent_header: Header,
}

#[derive(Deserialize, Debug)]
pub struct StatusResponseDetails {
    content: StatusContent,
    header: Header,
    metadata: Metadata,
    parent_header: Header,
}
