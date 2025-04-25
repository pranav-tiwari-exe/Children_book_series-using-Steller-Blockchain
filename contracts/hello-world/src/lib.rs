#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Book {
    pub id: u64,
    pub title: String,
    pub author: String,
    pub published_year: u64,
    pub is_available: bool,
}

#[contracttype]
pub enum BookKey {
    Book(u64),
}

// Shortened symbol to comply with Soroban's 9-character limit
const BOOK_CNT: Symbol = symbol_short!("BOOK_CNT");

#[contract]
pub struct ChildrenBookContract;

#[contractimpl]
impl ChildrenBookContract {
    // Add a new book to the contract storage
    pub fn add_book(env: Env, title: String, author: String, year: u64) -> u64 {
        let mut book_count: u64 = env.storage().instance().get(&BOOK_CNT).unwrap_or(0);
        book_count += 1;

        let new_book = Book {
            id: book_count,
            title,
            author,
            published_year: year,
            is_available: true,
        };

        env.storage().instance().set(&BookKey::Book(book_count), &new_book);
        env.storage().instance().set(&BOOK_CNT, &book_count);
        book_count
    }

    // Retrieve a book's details by ID
    pub fn get_book(env: Env, book_id: u64) -> Book {
        env.storage().instance().get(&BookKey::Book(book_id)).unwrap_or(Book {
            id: 0,
            title: String::from_str(&env, "Not Found"),
            author: String::from_str(&env, "Unknown"),
            published_year: 0,
            is_available: false,
        })
    }

    // Mark a book as unavailable
    pub fn mark_unavailable(env: Env, book_id: u64) {
        let mut book = Self::get_book(env.clone(), book_id);
        book.is_available = false;
        env.storage().instance().set(&BookKey::Book(book_id), &book);
    }

    // Return the total number of books added
    pub fn get_book_count(env: Env) -> u64 {
        env.storage().instance().get(&BOOK_CNT).unwrap_or(0)
    }
}