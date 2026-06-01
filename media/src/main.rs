// We can pass or take Media as an argument 
#[derive(Debug)]
enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String}
}

impl Media {
    fn description(&self) -> String {
        
        // Manually check for the types
        // if let Media::Book {title, author} = self{
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie {title, director} = self{
        //     format!("Movie: {} {}", title, director)
        // } else if let Media::Audiobook {title} = self{
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Unknown")
        // }
        
        // Pattern matching
        match self {
            Media::Book {title, author} => {
                format!("Book: {} {}", title, author)
            },
            Media::Movie {title, director} => {
                format!("Movie: {} {}", title, director)
            },
            Media::Audiobook {title} => {
                format!("Audiobook: {}", title)
            }
        }

        
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook{
        title: String::from("An audiobook"),
    };
    let goodmovie = Media::Movie{
        title: String::from("Good movie"),
        director: String::from("Good movie")
    };
    let badbook = Media::Book{
        title: String::from("bad book"),
        author: String::from("bad author")
    };

    println!("{}", audiobook.description());
    println!("{}", goodmovie.description());
    println!("{}", badbook.description());

    print_media(audiobook);
    print_media(goodmovie);
    print_media(badbook);
}
