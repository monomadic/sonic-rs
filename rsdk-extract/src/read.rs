use crate::container::RSDKContainer;
use nom::IResult;

pub(crate) fn container<'a>(i: &'a [u8]) -> IResult<&[u8], RSDKContainer> {
    nom::combinator::map(nom::bytes::complete::tag("RSDKvB"), |_| RSDKContainer {})(i)
}
