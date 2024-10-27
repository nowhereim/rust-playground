-- Your SQL goes here
   -- up.sql
   CREATE TABLE tests (
       id INTEGER PRIMARY KEY AUTO_INCREMENT,
       name VARCHAR(255) NOT NULL,
       email VARCHAR(255) NOT NULL UNIQUE
   );

   -- down.sql
   DROP TABLE tests;