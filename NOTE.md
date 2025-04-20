mod
- if you declare mod aaa, rust will looks for a file name aaa.rs or a directory named aaa with mod.rs
- we can explicitly set the path of the mod like below

#[path = "geometry/mod123.rs"]
mod geometry

How to try a file in rust, there are 2 ways to do it:
1. Run it on existing main.rs
  1. make a separate file and export it as a module, make sure the function is public
  2. use the mod keyword to declare a module and then call the function directly
  3. impor the module using the use keyword, this is useful when you have a lot of modules and you want to import them all at once

2. Keep the file as a separate file and run it as a standalone program
  1. make sure the file has a main function, this is the entry point of the program
  2. run the file using cargo run --bin <path_of_the_file_name> or rustc <file_name>.rs && ./<file_name>
     (cargo identifies binaries by their name, not by their extension)

command to list available binaries in the current project
cargo run --bin


Arc (Atomic Reference Counted Pointer)
- Arc is a thread-safe reference-counting pointer. It allows multiple threads to own the same data, and it keeps track of how many references there are to that data. When the last reference is dropped, the data is deallocated.
- Arc is used when you need to share data between threads and ensure that the data is not deallocated while it is still being used by other threads. It is a smart pointer that provides shared ownership of data.

Mutex (Mutual Exclusion)
- Mutex is a synchronization primitive that provides mutual exclusion, allowing only one thread to access the data at a time. It is used to protect shared data from being accessed by multiple threads simultaneously
- Mutex is used when you need to ensure that only one thread can access a piece of data at a time. It is a synchronization primitive that provides mutual exclusion, allowing only one thread to access the data at a time.




--------------------------------------------------------
list of question 
1. Can you give me a list of difference between mysql and graphql?
MySQL:
* What it is: MySQL is a Relational Database Management System (RDBMS).
* Purpose: Its primary job is to store, manage, organize, and retrieve data persistently. Think of it as the actual storage container and manager for your application's data.
* Layer: It operates at the data persistence layer (the backend's backend).
* Data Model: Uses a relational model. Data is organized into tables, which contain rows (records) and columns (attributes). Relationships between tables are defined using keys (primary and foreign keys).
* Query Language: Uses SQL (Structured Query Language). SQL is a powerful language specifically designed for interacting with relational databases – creating tables, inserting data, updating data, deleting data, and querying data (e.g., SELECT, INSERT, UPDATE, DELETE).
* How data is fetched: You write SQL queries that specify which tables, columns, and rows you want, often involving JOIN operations to combine data from multiple tables. The database server executes this query and returns the data exactly as requested by the SQL statement, typically entire rows or specified columns that match the criteria.

GraphQL:
* What it is: GraphQL is a query language for APIs and a runtime for fulfilling those queries with your existing data.
* Purpose: Its primary job is to define a way for clients (like web or mobile apps) to request exactly the data they need from a server, and nothing more. It acts as an interface or contract between the client and the server's data logic.
* Layer: It operates at the API layer, sitting between the client application and the backend server logic (which might, in turn, interact with a database like MySQL).
* Data Model: Uses a graph-based model defined by a schema. This schema describes the types of data available (Objects, Fields, Scalars) and the relationships between them. It represents your data as a graph of interconnected objects.
* Query Language: Uses its own query language (GraphQL). Clients send GraphQL queries that mirror the shape of the data they want back. These queries specify precisely which fields on which objects they need.
* How data is fetched:
    1.  A client sends a GraphQL query to a GraphQL server endpoint.
    2.  The GraphQL server parses the query and validates it against the schema.
    3.  For each field requested in the query, the server executes a corresponding "resolver" function.
    4.  These resolvers are the backend code you write. They contain the logic to fetch the actual data for that specific field – this logic might involve querying a database (like MySQL using SQL), calling another microservice, or accessing a cache.
    5.  The GraphQL serve

------------------------------------
BE Web Framework
  Rocket

DB ORM
  Diesel

FE Web Framework
