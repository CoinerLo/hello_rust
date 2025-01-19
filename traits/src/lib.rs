// когда требуется обязательная имплементация метода в структуре которая реализуется этот типаж
pub trait Summary {
    fn summarize(&self) -> String;
}

// имплементация метода по дефолту
pub trait Summary_def {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct SMS {
    pub content: String,
    pub number: String,
}

impl Summary_def for SMS {} // имплементация типажей с дефолтными методами

pub trait Summary_v2 {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // метод обаращется к методам своего типажа и становится заданным по дефолту
    }                                                              // для структуры достаточно будет имплементировать только summarize_author
}

pub struct Email {
    pub address: String,
    pub content: String,
}

pub struct Tweet2 {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary_v2 for Email {
    fn summarize_author(&self) -> String {
        println!("Only address...");
        self.address.clone()
    }
}

impl Summary_v2 for Tweet2 {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn test_run() {
    let mut _i = Tweet {
        username: String::from("My name"),
        content: String::from("Hello my content"),
        reply: false,
        retweet: false,
    };
    println!("call summarize = {}", _i.summarize());

    let mut _sms = SMS {
        content: String::from("Hello my sms"),
        number: String::from("+7999"),
    };
    println!("call default summarize = {}", _sms.summarize());

    let email = Email {
        address: String::from("Not found"),
        content: String::from("Not yet"),
    };
    println!("call special summarize for email = {}", email.summarize());
}

#[cfg(test)]
mod tests {

}
