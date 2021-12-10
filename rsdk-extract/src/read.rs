use crate::container::RSDKContainer;
use nom::IResult;

pub(crate) fn container<'a>(i: &'a [u8]) -> IResult<&[u8], RSDKContainer> {
    nom::combinator::map(header, |(_, s)| {
        println!("Pack count: {}", s);
        RSDKContainer::default()
    })(i)
}

fn header(i: &[u8]) -> IResult<&[u8], (&[u8], u16)> {
    nom::sequence::tuple((
        nom::bytes::complete::tag("RSDKvB"),
        nom::number::complete::le_u16,
    ))(i)
}
