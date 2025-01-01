use prost::Message;

use crate::error::Error;

use super::v1::{
    request, 
    response, 
    Block, 
    BlockHeightReq, 
    BlockHeightResp, 
    BlocksReq, 
    BlocksResp, 
    Method, 
    Request, 
    Response
};

impl Request {
    pub fn new_block_height_request() -> Self {
        Request {
            method: Method::Height as i32,
            body: Some(request::Body::BlockHeightReq(BlockHeightReq{}))
        }
    } 

    pub fn new_blocks_request(from_height: u64) -> Self {
        Request { 
            method: Method::Blocks as i32, 
            body: Some(request::Body::BlocksReq(BlocksReq{from_height}))
        }
    }
}

impl Response {
    pub fn new_block_height_response(height: u64) -> Self {
        Response { 
            method: Method::Height as i32, 
            body: Some(response::Body::BlockHeightResp(BlockHeightResp{height})) 
        }
    }

    pub fn new_blocks_response(blocks: Vec<Block>) -> Self {
        Response{
            method: Method::Blocks as i32,
            body: Some(response::Body::BlocksResp(BlocksResp{blocks}))
        }
    }
}

impl TryFrom<Vec<u8>> for Request {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::decode(value.as_slice())?)
    }
}

impl From<Request> for Vec<u8> {
    fn from(value: Request) -> Self {
        value.encode_to_vec()
    }
}

impl TryFrom<Vec<u8>> for Response {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::decode(value.as_slice())?)
    }
}

impl From<Response> for Vec<u8> {
    fn from(value: Response) -> Self {
        value.encode_to_vec()
    }
}

impl TryFrom<Response> for BlockHeightResp {
    type Error = Error;
    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value.body {
            Some(body) => {
                match body {
                    response::Body::BlockHeightResp(resp) => Ok(resp),
                    response::Body::BlocksResp(_) => Err(Error::InvalidResponse),
                }
            },
            None => Err(Error::InvalidResponse)
        }
    }
}

impl TryFrom<Response> for BlocksResp {
    type Error = Error;
    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value.body {
            Some(body) => {
                match body {
                    response::Body::BlockHeightResp(_) => Err(Error::InvalidResponse),
                    response::Body::BlocksResp(resp) => Ok(resp),
                }
            }
            None => Err(Error::InvalidResponse)
        }
    }
}
