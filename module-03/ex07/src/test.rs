
#[cfg(test)]
mod tests {
    use crate::*;
    // #[derive(Debug, PartialEq)]
    // struct User {
    //     name: String,
    //     age: u32,
    // }
    //
    // impl Record for User { /* ... */ }
    //
    // #[test]
    // fn test_encode() {
    //     let database = [
    //         User { name: "aaa".into(), age : 23 },
    //         User { name: "bb".into(), age: 2 },
    //     ];
    //
    //     let csv = encode_csv(&database).unwrap();
    //
    //     assert_eq!(
    //         csv,
    //         "\
    //     aaa,23\n\
    //     bb,2\n\
//         "
//         );
//     }
//
//     #[test]
//     fn test_decode() {
//         let csv = "\
//         hello,2\n\
//         yes,5\n\
//         no,100\n\
//     ";
//
//         let database: Vec<User> = decode_csv(csv).unwrap();
//
//         assert_eq!(
//             database,
//             [
//                 User { name: "hello".into(), age: 2 },
//                 User { name: "yes".into(), age: 5 },
//                 User { name: "no".into(), age: 100 },
//             ]
//         );
//     }
//
//     #[test]
//     fn decoding_error() {
//         let csv = "\
//         hello,2\n\
//         yes,6\n\
//         no,23,hello\n\
//     ";
//
//         decode_csv::<User>(csv).unwrap_err();
//     }
}