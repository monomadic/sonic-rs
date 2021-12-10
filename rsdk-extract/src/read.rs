use crate::container::RSDKContainer;
use nom::IResult;

pub(crate) fn container<'a>(i: &'a [u8]) -> IResult<&[u8], RSDKContainer> {
    nom::combinator::map(header, |(_, s)| {
        println!("File count: {}", s);
        RSDKContainer::default()
    })(i)
}

//** Returns header tag + file count. */
fn header(i: &[u8]) -> IResult<&[u8], (&[u8], u16)> {
    nom::sequence::tuple((
        nom::bytes::complete::tag("RSDKvB"),
        nom::number::complete::le_u16,
    ))(i)
}
