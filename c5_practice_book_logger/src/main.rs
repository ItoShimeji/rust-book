struct Book {
    title: String,
    author: String,
    pages: u32,
    current_page: u32,
}

impl Book {
    // 例: 「吾輩は猫である」 by 夏目漱石, 320 pages
    fn summary(&self) -> String {
        format!(
            "「{}」 by {}, {} pages",
            self.title, self.author, self.pages
        )
    }

    fn progress(&self) -> f64 {
        // 0 除算の回避
        let rate = if self.pages == 0 {
            1.0
        } else {
            self.current_page as f64 / self.pages as f64
        };

        rate * 100.0
    }

    fn read(&mut self, pages: u32) {
        if self.current_page + pages <= self.pages {
            self.current_page += pages;
        } else {
            self.current_page = self.pages;
        }
    }

    fn is_finished(&self) -> bool {
        self.current_page >= self.pages
    }

    fn new(title: String, author: String, pages: u32) -> Book {
        Book {
            title,
            author,
            pages,
            current_page: 0,
        }
    }
}

fn main() {
    let mut book = Book::new(
        String::from("The Rust Programming Language"),
        String::from("Steve Klabnik and Carol Nichols"),
        500,
    );

    println!("{}", book.summary());
    println!("progress: {}%", book.progress());

    book.read(120);
    println!("progress: {}%", book.progress());

    book.read(1000);
    println!("finished: {}", book.is_finished());
    println!("current_page: {}", book.current_page);
}
