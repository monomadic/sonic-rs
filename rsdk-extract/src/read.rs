use crate::container::{FileDescriptionBlock, RSDKContainer};
use nom::IResult;

// pub(crate) fn container<'a>(i: &'a [u8]) -> IResult<&[u8], RSDKContainer> {
//     nom::sequence::tuple((
//         nom::combinator::map(header, |(_, s)| {
//             println!("File count: {}", s);
//             RSDKContainer::default()
//         }),
//         file,
//     ))(i)
// }

pub(crate) fn container<'a>(i: &'a [u8]) -> IResult<&[u8], RSDKContainer> {
    nom::combinator::map(
        nom::sequence::tuple((header, nom::multi::many1(file_index))),
        |(_header, files)| RSDKContainer { files },
    )(i)
}

//** Returns header tag + file count. */
fn header(i: &[u8]) -> IResult<&[u8], (&[u8], u16)> {
    nom::sequence::tuple((
        nom::bytes::complete::tag("RSDKvB"),
        nom::number::complete::le_u16,
    ))(i)
}

//** Returns hash, offset, size. */
fn file_index(i: &[u8]) -> IResult<&[u8], FileDescriptionBlock> {
    nom::combinator::map(
        nom::sequence::tuple((
            nom::bytes::complete::take(4_usize),
            nom::number::complete::le_u32,
            nom::number::complete::le_u32,
        )),
        |(_checksum, offset, size)| FileDescriptionBlock { offset, size },
    )(i)
}
