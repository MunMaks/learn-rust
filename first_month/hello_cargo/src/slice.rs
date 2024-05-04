

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];

    for i in slice{
        println!("i: {}", i);
    }
    assert_eq!(slice, &[2, 3]);

}


// fn main() {
//     let mut s: String = String::from("Gustave Eiffel"); 
//     let longueur: usize = first_word(&s);
//     println!("{s}, len: {}", longueur);

//     s.clear(); // this empties the String, making it equal to ""


//     let s1 = String::from("hello world");

//     let hello: &str = &s1[0..];
//     let world: &str = &s1[6..11];

//     println!("{}, {}", hello, world);

// }


// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }


// return la longueur du premier mot rencontrew
// fn first_word(s: &String) -> usize {
//     let bytes: &[u8] = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }


// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }


/*

String:
    Heap-Allocated: String is a dynamically-sized, mutable, heap-allocated string.
        It can grow or shrink as needed, making it suitable for dynamic text processing.
    Creation: You can create a String from a string literal using the 
        to_string() method or the String::from() function.
    Manipulation: String offers a variety of methods for manipulating text, 
        including push(), push_str(), and insert().
    Ownership: Being an owned type, a String owns its data and is responsible for 
        deallocating it when it goes out of scope.


&str:
    Borrowed Slice: &str is a borrowed slice of a string.
        It's immutable and represents a view into a string or string literal.
    Efficient: Since &str is a reference to existing string data,
        it's memory-efficient and suitable for lightweight string operations.
    Lifetimes: Because &str is a reference, its lifetime is tied to the data
        it refers to, making it important to consider lifetimes when working with &str.


Cow<str>:
    Copy-On-Write: Cow<str> is a smart pointer that provides a copy-on-write string.
        It can be either a borrowed &str or an owned String.
    Flexibility: Cow<str> is useful when you want to avoid copying string data unnecessarily
        but still need the ability to create a mutable copy if needed.
    Conversion: Cow<str> automatically converts from a &str or String, making it a flexible
        choice for functions that need to accept either type.


OsString and OsStr:
    Operating System Compatibility: OsString and OsStr are designed to handle platform-specific
        text encodings, such as those used by file paths and environment variables.
    Usage: OsString is an owned, mutable string similar to String, while OsStr is a borrowed,
        immutable slice similar to &str.
    Conversion: Functions like OsString::from() and OsStr::to_str() help convert between these
        types and standard Rust strings.


PathBuf and Path:
    File Paths: PathBuf and Path are specialized string types for handling file paths
        in a cross-platform manner.
    Usage: PathBuf is an owned, mutable type, while Path is a borrowed slice that
        provides various methods for path manipulation.
    Conversions: You can convert between these types and standard Rust strings using
        methods like Path::to_str().


Other String Types:
    Rust's standard library and ecosystem include additional specialized string types,
        such as CString and CStr, which are designed for interoperability with C-style strings.
    These types provide methods for safely handling null-terminated strings and converting
        to and from standard Rust strings.


*/