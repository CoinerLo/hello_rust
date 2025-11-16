struct Author<'a> {
    name: &'a str,
    birth_year: u16
}

struct Book<'a> {
    title: &'a str,
    year: u16,
    author: Author<'a>,
    tags: Vec<&'a str>,
    copies: Vec<{ (id: u32, availability: bool) }>
}
struct Library<'a> {
    books: Vec<Book<'a>>
}

fn count_available_copies(book: &Book) -> usize {
    let count = book.copies.filter(|exem| exem.availability);
    count.len()
}

// fn find_books_by_author(library: &Library, name: &str) -> Box<[&Book]> {
//     // Ваш код
// }

// fn add_tag(book: &mut Book, tag: &str) {
//     // Ваш код
// }

// fn oldest_book(library: &Library) -> Option<&Book> {
//     // Ваш код
// }

fn main() {
    let mut book1 = Book {
        title: "Солярис",
        year: 1961,
        author: Author { name: "Лем", birth_year: 1921 },
        tags: vec!["sci-fi"],
        copies: vec![(1, true), (2, false), (3, true)],
    };

    // add_tag(&mut book1, "classic");
    // add_tag(&mut book1, "sci-fi");

    let book2 = Book {
        title: "Пикник на обочине",
        year: 1972,
        author: Author { name: "Стругацкие", birth_year: 1933 },
        tags: vec!["sci-fi", "classic"],
        copies: vec![(10, false), (11, false)],
    };

    let library = Library {
        books: vec![book1, book2],
    };

    // let books_lem = find_books_by_author(&library, "Лем");
    // assert_eq!(books_lem.len(), 1);
    // assert_eq!(books_lem[0].title, "Солярис");

    // let books_str = find_books_by_author(&library, "Стругацкие");
    // assert_eq!(books_str.len(), 1);
    // assert_eq!(books_str[0].title, "Пикник на обочине");

    // let oldest = oldest_book(&library).unwrap();
    // assert_eq!(oldest.title, "Солярис");

    let first_book = &library.books[0];
    assert_eq!(count_available_copies(first_book), 2);

    println!("Все тесты прошли!");
}
