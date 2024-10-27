-- Your SQL goes here
   -- up.sql
   CREATE TABLE tests (
       id INT PRIMARY KEY AUTO_INCREMENT,
       name VARCHAR(255) NOT NULL,
       email VARCHAR(255) NOT NULL UNIQUE
   );